use std::{collections::HashSet, sync::{Arc,}};
use std::sync::atomic::{AtomicUsize,Ordering};
use scraper::{Html, Selector};
use::tokio::sync::{mpsc,Mutex};
use std::time::{Instant};

use crate::models::crawl_node::CrawlNode;
// use url::Url;
use url::Url;







pub async fn crawl(url:String)-> Result<HashSet<String>, reqwest::Error>{
    let start = Instant::now();

    let visited_links = Arc::new(Mutex::new(HashSet::new()));
    let (tx,mut rx)= mpsc::channel::<CrawlNode>(100);
    let base_url= Url::parse(&url).unwrap();
    let counter = Arc::new(AtomicUsize::new(0));

    let page_limit=5;

    // Send the initial URL to start crawling
    let initial_node = CrawlNode {
        url: url.clone(),
        parent: None,
    };

    let _ = tx.send(initial_node).await;


    while let Some(node)=rx.recv().await {
        let visited_links = Arc::clone(&visited_links);
        let tx=tx.clone();
        let base_url =base_url.clone();
        let counter = Arc::clone(&counter);

        tokio::spawn(async move {
            let count = counter.fetch_add(1, Ordering::SeqCst);

            if count >= page_limit {
                return;
            }
            let mut visited  = visited_links.lock().await;

            if visited.contains(&node.url) {
                return;
            }

            visited.insert(node.url.clone());
            let Ok((raw_html, raw_string)) = fetch_html(&node.url).await else { return; };

            println!("Visiting 🌐 {}", &node.url);
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
                        let _ = tx.send(new_node).await;
                    }
                }
            }
        });
    }

    let visited = visited_links.lock().await;
    let elapsed_time = start.elapsed();
    print!("With thread:{:?}",elapsed_time);
    Ok(visited.clone())
}




async fn fetch_html (url:&str)-> Result<(Html,String),reqwest::Error>{


    let string_body = reqwest::get(url)
    .await?
    .text()
    .await?;
    
   let framgmnet = Html::parse_fragment(&string_body);

 

   // println!("Fragments: {:?}",framgmnet);
   
   Ok((framgmnet,string_body))
   
}


pub fn extract_links(framgmnet:Html)->HashSet<String> {

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
