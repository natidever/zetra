// //     // Top Level Directory

mod models;
mod crawler;
mod utils;
mod constants;
mod config;
// use models::crawl_node::CrawlNode;
// use crawler::engine::{extract_links,crawl,};




use std::sync::{Arc, Mutex};

use tokio;
use reqwest;
use scraper::{Html,Selector };

use crate::crawler::threaded_engine;




// #[tokio::main]
// async fn main() {

//   let body = r#"
//         <html>
//             <body>
//                 <a href="https://example.com">Example</a>
//                 <a href="/about">About</a>
//             </body>
// //         </html>
// //     "#;

// //  let parsed_html = Html::parse_document(body);
// //  extract_links(parsed_html);
//  let gomeraw = crawl("https://www.rust-lang.org/".to_string()).await;
// //  let gomeraw = crawl("https://www.rust-lang.org/".to_string()).await;


//  println!("Gomeraw: {:?} ",{gomeraw});




















// //   let start_url = "https://www.rust-lang.org/".to_string();

//       let start_url = "https://www.rust-lang.org/".to_string();

//     let result = crawl(start_url).await;

//     match result {
//         Ok(visited) => println!("Crawled {} pages.", visited.len()),
//         Err(e) => eprintln!("Crawl failed: {}", e),
//     }

//     // optional: explicitly return ()
//     ()


 
 
// }


use crawler::threaded_engine::{crawl};


#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting....");
    // let start_url = "https://www.rust-lang.org/".to_string();
    let start_url = "https://jiji.com.et/".to_string();


    
    let visited = crawl(start_url).await?;  // Use ? to unwrap or return error early

    println!("Crawled {} pages.", visited.len());

    Ok(())
}

// fn main (){
//     let ful_html_with_title =r#"
//         <html>
//             <head>
//                 <title>Example Title</title>
//             </head>
//             <body>
//                 <h1>Welcome to Example</h1>
//                 <p>This is an example page.</p>
//             </body>
//         </html>
//     "#;
//     let html = Html::parse_document(ful_html_with_title);

//     let is_page_analyzed = false;
//     first_page_analysis(&is_page_analyzed, &html);
// }






// Benchmark for with thread(green) and no thread

// with thread
// Crawled 20 pages in 9.185271777s





// No thread 122.100311041s
// No thread 158.0846338s
 