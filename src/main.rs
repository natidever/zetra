// Top Level Directory

mod models;
mod crawler;


use models::crawl_node::CrawlNode;
use crawler::engine::{extract_links,crawl};


use tokio;
use reqwest;
use scraper::{Html,Selector };




#[tokio::main]
async fn main() {

//   let body = r#"
//         <html>
//             <body>
//                 <a href="https://example.com">Example</a>
//                 <a href="/about">About</a>
//             </body>
//         </html>
//     "#;

//  let parsed_html = Html::parse_document(body);
//  extract_links(parsed_html);
 let gomeraw = crawl("https://www.rust-lang.org/".to_string()).await;
 println!("Gomeraw: {:?} ",{gomeraw})


 
 
}
