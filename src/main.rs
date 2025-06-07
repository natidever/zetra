//     // Top Level Directory

mod models;
mod crawler;


// use models::crawl_node::CrawlNode;
// // use crawler::engine::{extract_links,crawl,};
// use crawler::threaded_engine::{crawl};




// use tokio;
// use reqwest;
// use scraper::{Html,Selector };

// use crate::crawler::threaded_engine;




// #[tokio::main]
// async fn main() {

// //   let body = r#"
// //         <html>
// //             <body>
// //                 <a href="https://example.com">Example</a>
// //                 <a href="/about">About</a>
// //             </body>
// //         </html>
// //     "#;

// //  let parsed_html = Html::parse_document(body);
// //  extract_links(parsed_html);
// //  let gomeraw = crawl("https://www.rust-lang.org/".to_string()).await;
// //  let gomeraw = crawl("https://www.rust-lang.org/".to_string()).await;


// //  println!("Gomeraw: {:?} ",{gomeraw})




















//   let start_url = "https://www.rust-lang.org/".to_string();

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
    let start_url = "https://www.rust-lang.org/".to_string();

    let visited = crawl(start_url).await?;  // Use ? to unwrap or return error early

    println!("Crawled {} pages.", visited.len());

    Ok(())
}