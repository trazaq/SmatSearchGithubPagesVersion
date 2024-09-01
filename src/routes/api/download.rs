use crate::routes::api::errors::SearchError;
use crate::Configuration;
use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpRequest, HttpResponse};
use async_stream::stream;
use bytes::Bytes;
use futures::io;
use glob::glob;
use rusqlite::ErrorCode::NotADatabase;
use rusqlite::{Connection, OpenFlags, Statement};
use serde::{Deserialize, Serialize};
use time::macros::format_description;
use time::{Date, Duration};

#[tracing::instrument(skip(_req, config))]
#[get("/api/download")]
pub async fn download(
    _req: HttpRequest,
    config: web::Data<Configuration>,
    mut path: web::Query<IncomingSearchRequest>,
) -> Result<HttpResponse, SearchError> {
    // Set the date to today if empty
    if path.date1.is_none() {
        path.date1 = Some(chrono::offset::Local::now().date_naive().to_string());
    }

    if path.date2.is_none() {
        path.date2 = Some(chrono::offset::Local::now().date_naive().to_string());
    }

    let smatdbs = find_smatdb_files(&path, &config)?;

    let stream = stream! {
        let mut errors : Vec<String> = Vec::new();

        for smat in &smatdbs {
            let config = config.clone();
            let path = path.clone();

            let sql = prepare_sql(&path.clone(), &config.clone());

            let conn = get_sql_connection(smat)
                      .map_err(|e| {
                          errors.push(e.to_string());
                      }).unwrap();

            let pragma = format!(
                      "pragma key = '{}';pragma cipher_compatibility = {};",
                      path.site, config.SQL_CIPHER_COMPATIBILITY
                  );


            conn.execute(r#"PRAGMA case_sensitive_like=true;"#, []).ok();

            let mut stmt = prepare_statment(&conn, &sql, &pragma)
                          .map_err(|e| SearchError::InternalError(e.to_string()))
                          .unwrap();

            let mut rows = stmt.query([]).unwrap();

            // First send a single message
            if let Some(row) = rows.next().unwrap() {
                let msg = String::from_utf8_lossy(row.get_ref(0).unwrap().as_bytes().unwrap());
                yield Ok::<Bytes, io::Error>(Bytes::from(msg.as_ref().to_owned()));
            };

            // Then for the rest, add a LF that will separate the HL7 messages for Cloverleaf
            while let Some(row) = rows.next().unwrap() {
                let msg = String::from_utf8_lossy(row.get_ref(0).unwrap().as_bytes().unwrap());
                yield Ok::<Bytes, io::Error>(Bytes::from("\n"));
                yield Ok::<Bytes, io::Error>(Bytes::from(msg.as_ref().to_owned()));
            }

        }
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::octet_stream())
        .streaming(stream))
}

//#################### HELPER FUNCTIONS ###############################
//#####################################################################

fn get_sql_connection(path: &str) -> Result<Connection, rusqlite::Error> {
    Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
}

#[rustfmt::skip]
fn prepare_statment<'a>(conn: &'a Connection, sql: &str, pragma: &str, ) -> Result<Statement<'a>, SearchError> {
    let statement = conn.prepare(sql);
    if statement.is_err() && statement.as_ref().err().unwrap().sqlite_error_code() == Some(NotADatabase)
    {
        conn.execute_batch(pragma).map_err(|e| SearchError::InternalError(e.to_string()))?;
        match conn.prepare(sql) {
            Ok(s) => Ok(s),
            Err(e) => Err(SearchError::InternalError(e.to_string())),
        }
    } else {
        Ok(statement.map_err(|e| SearchError::InternalError(e.to_string()))?)
    }
}

/// Validates to the input dates.
/// Returns the valid date, else returns a 400 Response with error info
fn validate_date(date: &str) -> Result<Date, SearchError> {
    match Date::parse(
        date,
        format_description!(
            "[year repr:full]-[month repr:numerical padding:zero]-[day padding:zero]"
        ),
    ) {
        Ok(date) => Ok(date),
        Err(e) => Err(SearchError::BadClientData(format!(
            "Bad Input Date: {date}. Expected Format: <YYYY-MM-DD>. Error Info: {e}"
        ))),
    }
}

// We need to take the date range and convert it to a list of individual dates
// Then take those individual dates and glob for the files using each of them
// Glob works a bit differently than regex: [20-31] doesn't mean match each number twenty through thirty,
// rather it it will match anything with a 2, anything in the range 0-3, and anything with a 1. So it was returning more matches than expected.
// Which is why it was decided to match each file name explicitly, else a regex may have worked but the below does, too and is more straight forward(?)
#[rustfmt::skip]
fn find_smatdb_files(
    path: &web::Query<IncomingSearchRequest>,
    config: &web::Data<Configuration>,
) -> Result<Vec<String>, SearchError> {
    let mut results = vec![];
    let date1 = validate_date(path.date1.as_ref().unwrap())?;
    let date2 = validate_date(path.date2.as_ref().unwrap())?;
    // Convert the date range into a list of individual dates
    let mut date_list = vec![];
    let mut dt = date1;
    while dt <= date2 {
        date_list.push(dt);
        dt += Duration::days(1);
    }
    tracing::debug!("Date list: {:?}", &date_list);

    let today = chrono::offset::Local::now();
    //let today = chrono::offset::Local::now().date_naive().to_string();
    tracing::debug!("Today's date: {:?}", &today.date_naive().to_string());

    for query_date in date_list {
        // If the query date is today, these SMATDB files are named without the date portion
        let pattern =
            if query_date.to_string() == today.date_naive().to_string() {
                format!(
                    // :#02 is a format specifier to pad with leading zeroes
                    "{}/{}[.]smatdb",
                    config.SMAT_PATH
                        .replace("<HCIROOT>", &config.HCIROOT)
                        .replace("<SITE>", &path.site)
                        .replace("<PROCESS>", &path.process),

                    &path.smat_file,
                )
            } else {
                // Produces like: "tests/verity/exec/processes/verity/fr_verity.2023-05-10*.smatdb"
                format!(
                    // :#02 is a format specifier to pad with leading zeroes
                    // '**' pattern means search any subdirectories
                    "{}/**/{}[.]{}[-]{:#02}[-]{:#02}*[.]smatdb",
                    config.SMAT_PATH
                        .replace("<HCIROOT>", &config.HCIROOT)
                        .replace("<SITE>", &path.site)
                        .replace("<PROCESS>", &path.process),

                    &path.smat_file,
                    query_date.year(),
                    query_date.month() as u8,
                    query_date.day()
                )
            };

        tracing::debug!("Patterns: {:?}", &pattern);

        for entry in glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => results.push(path.to_string_lossy().to_string()),
                Err(e) => tracing::error!("GlobError: {:?}", e),
            }
        }
    }

    results.sort();
    tracing::info!("{:?}", &results);

    Ok(results)
}

/// Prepares the user's query by escaping certain characters
fn escape_special_characters(query: &str) -> String {
    query
        .replace('\'', "''")
        .replace('%', r#"\%"#)
        .replace('_', r#"\_"#)
        .replace('\\', r#"\\"#)
}

// the '\' escape is only used for '%' and '_' characters in the LIKE expression
// other characters, such as single quotes, must be escaped differently/separately
fn prepare_sql(
    path: &web::Query<IncomingSearchRequest>,
    config: &web::Data<Configuration>,
) -> String {
    if path.search.contains(&config.SQL_LOG_AND) {
        config
            .SQL_QUERY
            .replace(
                "<replaceme>",
                path.search
                    .split(&config.SQL_LOG_AND)
                    .map(|query| {
                        format!(
                            "'%{}%' {} AND MessageContent LIKE ",
                            escape_special_characters(query),
                            r#"ESCAPE '\'"#
                        )
                    })
                    .collect::<String>()
                    .trim_end_matches(" AND MessageContent LIKE "),
            )
            .replace("<LIMIT>", &path.msg_limit.to_string())
    } else if path.search.contains(&config.SQL_LOG_OR) {
        config
            .SQL_QUERY
            .replace(
                "<replaceme>",
                path.search
                    .split(&config.SQL_LOG_OR)
                    .map(|query| {
                        format!(
                            "'%{}%' {} OR MessageContent LIKE ",
                            escape_special_characters(query),
                            r#"ESCAPE '\'"#
                        )
                    })
                    .collect::<String>()
                    .trim_end_matches(" OR MessageContent LIKE "),
            )
            .replace("<LIMIT>", &path.msg_limit.to_string())
    } else {
        config
            .SQL_QUERY
            .replace(
                "<replaceme>",
                &format!(
                    "'%{}%' {}",
                    escape_special_characters(&path.search),
                    r#"ESCAPE '\'"#
                ),
            )
            .replace("<LIMIT>", &path.msg_limit.to_string())
    }
}

///////////////// TYPES ///////////////////////
///////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncomingSearchRequest {
    site: String,
    process: String,
    thread_name: String,
    date1: Option<String>,
    date2: Option<String>,
    search: String,
    msg_limit: u32,
    smat_file: String,
    download: Option<bool>,
}
#[cfg(test)]
mod tests {}
