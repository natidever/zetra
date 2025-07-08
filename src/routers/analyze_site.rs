use axum::{response::IntoResponse, routing::{post, Route}, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::error::{Result, Error};

pub async fn root() -> &'static str {
    "Zetra is coming soon! Stay tuned for updates."
}

 
pub fn routes()->  Router{

Router::new().route("/api/analyze-site", post(analyze_site_handler))
}


pub async fn analyze_site_handler(payload:Json<AuditSitePayload>)->Result<Json<Value>> {
   
     if payload.url.is_empty(){
        return Err(Error::InvalidUrl);
     }

     let response = Json(
        json!({
        "message": "Site audit completed successfully.",
        "url": payload.url,
     })
     );
    // let response = serde_json::json!({
    //     "message": "Site audit completed successfully.",
    //     "url": payload.url,
    // }); 
    Ok(response)


}

#[derive(Debug,Deserialize)]
struct AuditSitePayload {
    url: String,
}

