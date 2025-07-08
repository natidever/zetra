use std::{collections::HashSet, os::linux::raw, sync::Arc};
use std::sync::atomic::{AtomicUsize,Ordering};
use scraper::{Html, HtmlTreeSink, Selector};
use::tokio::sync::{mpsc,Mutex};
use std::time::{Instant};
use tokio::task::JoinSet;

use crate::models::crawl_node::CrawlNode;
use crate::config::crawler_config::CrawlerConfig;

use crate::utils::html_utils::first_page_analysis;
// use url::Url;
use url::Url;
// 
use tokio::task;





pub async fn crawl(url: String) -> Result<HashSet<String>, reqwest::Error> {

     let crawl_confg=CrawlerConfig::default();


    let start = Instant::now();

    let visited_links = Arc::new(Mutex::new(HashSet::new()));

    let is_first_page_analyised = Arc::new(Mutex::new(false));

    let (tx, mut rx) = mpsc::channel::<CrawlNode>(100);
    let base_url = Url::parse(&url).unwrap();
    let counter = Arc::new(AtomicUsize::new(0));
    let page_limit = crawl_confg.page_limt;

    let initial_node = CrawlNode {
        url: url.clone(),
        parent: None,
    };
    tx.send(initial_node).await.unwrap();



    while let Some(node) = rx.recv().await {
        let visited_links = Arc::clone(&visited_links);
        let tx = tx.clone();
        let base_url = base_url.clone();
        let counter = Arc::clone(&counter);

        // 🔒 Check + insert visited inside lock to avoid duplicate crawls
        let should_crawl = {
            let mut visited = visited_links.lock().await;
            if visited.contains(&node.url) || counter.load(Ordering::SeqCst) >= page_limit {
                false
            } else {
                visited.insert(node.url.clone());
                counter.fetch_add(1, Ordering::SeqCst);
                true
            }
        };

        if !should_crawl {
            continue;
        }
        let is_fp_analyzd=is_first_page_analyised.clone();
        // Only crawl if the link hasn't been visited and we're under limit
        task::spawn(async move {
            println!("Visiting🌐 {}", &node.url);
            
            let Ok((string_format)) = fetch_html(&node.url).await else { return; };
            

       
           first_page_analysis(&is_fp_analyzd,&string_format).await;

           let raw_html = Html::parse_document(&string_format);


            

            
           


            

            

            // Extract links from the HTML fragment
            
            
            let links = extract_links(raw_html);

            for link in links {
                let abs_url = Url::parse(&link).or_else(|_| {
                    Url::parse(&node.url).and_then(|base| base.join(&link))
                });

                if let Ok(abs) = abs_url {
                    if abs.domain() == base_url.domain() {
                        let new_node = CrawlNode {
                            url: abs.to_string(),
                            parent: Some(node.url.clone()),
                        };
                        // ⚠️ Don’t block if channel is closed
                        let _ = tx.send(new_node).await;
                    }
                }
            }
        });
        
        // ✅ Exit condition: limit reached
        if counter.load(Ordering::SeqCst) >= page_limit {
            break;
        }
    }

    // Allow final tasks to complete (optional delay or task tracking)
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let visited = visited_links.lock().await;
    println!("Crawled {} pages in {:?}", visited.len(), start.elapsed());

    Ok(visited.clone())
}


async fn fetch_html (url:&str)-> Result<String ,reqwest::Error>{


    let string_body = reqwest::get(url)
    .await?
    .text()
    .await?;
    
   let framgmnet = Html::parse_fragment(&string_body);
   
   

 

   // println!("Fragments: {:?}",framgmnet);
   
   Ok((string_body))
   
}


pub fn extract_links(framgmnet: Html)->HashSet<String> {

 let mut links:HashSet<String> = HashSet::new();
  let selector = Selector::parse("a").unwrap();

   for link in framgmnet.select(&selector){
      if let Some(n)=link.value().attr("href"){
          links.insert(n.to_string());
         // println!("link:{}",n);
      }
        
   }
  
  links

}















