use crate::{AppState, Configuration};
use actix_web::get;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use std::io;

#[get("/api/all_sites")]
pub async fn all_sites(
    _req: HttpRequest,
    _config: web::Data<Configuration>,
    _state: web::Data<AppState>,
) -> Result<HttpResponse, io::Error> {
    let sites = "t_device_in,t_device_out,adt_vpn,alladt,clin_vpn,clin_vpn1,consult_epic,dev_amisha,dev_bela,dev_chris,master,mshepic,msqepic,oneepicadt,oneepicchg,smatdb_template,smatfile_template,t_rad,taps,tbiegl,tbislraps,tbislrchg,tbislregl,tbislregl1,tbislrlab,tbislrlab1,tbislrvpn,tbislwepic,tcharge,tcharge1,teagle,tepic_in,tepic_in1,tepic_out,tepic_out1,tepicadt,tidm,tidx1,tidx2,tidx3,tidxflow,tlab,tlab5,tlabadt,tmshq,tmshq1,tnyee,tpathology,tprod,tprod1,tprod2,tprod3,tprod4,tprod5,tpyxis,tqry,trad_in,trad_out,tslegl,tsnch,tst_jim,twebapps,verity,verity_bi,verity_slr,vpn,vpn1,ws_adv_samples,ws_samples,dev_hari,t_epicadthcitcl";

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(sites.to_string()))
}
