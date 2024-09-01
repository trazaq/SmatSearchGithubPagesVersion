use chrono::{DateTime, Datelike, Local};
use gethostname::gethostname;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::Mutex;
use tokio_stream::wrappers::ReceiverStream;

pub mod route_config;
pub mod routes;
pub mod test_helpers;

#[allow(non_snake_case)]
pub struct Configuration {
    pub SQL_CIPHER_COMPATIBILITY: String,
    pub SQL_LOG_AND: String,
    pub SQL_LOG_OR: String,
    pub SQL_QUERY: String,
    pub HCIROOT: String,
    pub SMAT_PATH: String,
    pub SMAT_HISTORY_PATH: String,
    pub ENVIRONMENT: String,
    pub HOST: String,
    pub PORT: String,
    pub TITLE: String,
    pub TABLE_CELL_COLOR: String,
    pub BACKGROUND_COLOR_RGB: String,
    pub TEXT_COLOR: String,
    pub SERVER_INI: String,
    pub CONNLIST: String,
}

pub fn get_config() -> Configuration {
    Configuration {
        SQL_CIPHER_COMPATIBILITY: dotenvy::var("SQL_CIPHER_COMPATIBILITY")
            .expect("Error: Set the 'SQL_CIPHER_COMPATIBILITY' key in .env"),
        SQL_LOG_AND: dotenvy::var("SQL_LOG_AND").expect("Error: Set the 'SQL_LOG_AND' key in .env"),
        SQL_LOG_OR: dotenvy::var("SQL_LOG_OR").expect("Error: Set the 'SQL_LOG_OR' key in .env"),
        SQL_QUERY: dotenvy::var("SQL_QUERY").expect("Error: Set the 'SQL_QUERY' key in .env"),
        HCIROOT: dotenvy::var("HCIROOT").expect("Error: Set the 'HCIROOT' key in .env"),
        SMAT_PATH: dotenvy::var("SMAT_PATH").expect("Error: Set the 'SMAT_PATH' key in .env"),
        SMAT_HISTORY_PATH: dotenvy::var("SMAT_HISTORY_PATH")
            .expect("Error: Set the 'SMAT_HISTORY_PATH' key in .env"),
        ENVIRONMENT: dotenvy::var("ENVIRONMENT").expect("Error: Set the 'ENVIRONMENT' key in .env"),
        HOST: dotenvy::var("HOST").unwrap_or(gethostname().to_string_lossy().to_string()),
        PORT: dotenvy::var("PORT").expect("Error: Set the 'PORT' key in .env"),
        TITLE: dotenvy::var("TITLE").expect("Error: Set the 'TITLE' key in .env"),
        TABLE_CELL_COLOR: dotenvy::var("TABLE_CELL_COLOR")
            .expect("Error: Set the 'TABLE_CELL_COLOR' key in .env"),
        BACKGROUND_COLOR_RGB: dotenvy::var("BACKGROUND_COLOR_RGB")
            .expect("Error: Set the 'BACKGROUND_COLOR_RGB' key in .env"),
        TEXT_COLOR: dotenvy::var("TEXT_COLOR").expect("Error: Set the 'TEXT_COLOR' key in .env"),
        SERVER_INI: dotenvy::var("SERVER_INI")
            .expect("Error: The full path to the server.ini file must be set in .env"),
        CONNLIST: dotenvy::var("CONNLIST")
            .expect("Error: The full path to the connList.tcl file must be set in .env"),
    }
}

// Used for holding the Cloverleaf threads
#[derive(Debug)]
pub struct AppState {
    all_threads: Mutex<String>,
    last_updated: Mutex<DateTime<Local>>,
}

impl AppState {
    pub fn new() -> Self {
        let conn_list = Command::new(dotenvy::var("CONNLIST").expect("SET CONNLIST IN .env"))
            .output()
            .expect("Failed to execute connList.tcl")
            .stdout;
        let conn_list = String::from_utf8_lossy(&conn_list);

        AppState {
            all_threads: Mutex::new(conn_list.to_string()),
            last_updated: Mutex::new(Local::now()),
        }
    }
    pub fn refresh_threads(&self) {
        let last_updated_day = self.last_updated.lock().unwrap().day();
        let current_dt = Local::now();
        if last_updated_day != current_dt.day() {
            let conn_list = Command::new(dotenvy::var("CONNLIST").expect("SET CONNLIST IN .env"))
                .output()
                .expect("Failed to execute connList.tcl")
                .stdout;

            let conn_list = String::from_utf8_lossy(&conn_list);

            {
                *self.all_threads.lock().unwrap() = conn_list.to_string();
                *self.last_updated.lock().unwrap() = current_dt;
            }
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}



// Used in search_results.rs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncomingSearchRequest {
    site: String,
    process: String,
    thread_name: String,
    date1: Option<String>,
    date2: Option<String>,
    search: String,
    msg_limit: u64,
    smat_file: String,
    case_sensitive: Option<bool>
}

pub struct ResultStream {
    pub rx: ReceiverStream<Box<str>>,
}
// Using tokio provided ReceiverStream now
/*impl Stream for ResultStream {
    type Item = Result<Bytes, io::Error>;
    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // let this = self.get_mut();
        match self.rx {
            Ok(message) => Poll::Ready(Some(Ok(Bytes::copy_from_slice(message.as_bytes())))),
            Err(_) => Poll::Ready(None),
        }
    }
}*/