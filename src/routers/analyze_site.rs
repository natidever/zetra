use axum::{response::IntoResponse, routing::{post, Route}, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::{crawler::threaded_engine::crawl, error::{Error, Result}};

use redis::AsyncCommands;






















































pub async fn root() -> &'static str {
    "Zetra is coming soon! Stay tuned for updates."
}

 
pub fn routes()->  Router{

Router::new().route("/api/analyze-site", post(analyze_site_handler))
}


pub async fn analyze_site_handler(payload:Json<AuditSitePayload>)->Result<Json<Value>> {

    // if let Some(url)=payload.url{
    //     let visited = crawl(payload.url).await.unwrap();

    // }


   
    
    //  get title recommnedation
     let response = Json(
        json!({
        "message": "Site audit completed successfully.",
        "url": payload.url,
     })
     );
 
    Ok(response)


}

#[derive(Debug,Deserialize)]
struct AuditSitePayload {
    url: String,
}

