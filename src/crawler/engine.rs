
#![allow(unused_variables)]

use crate::models::crawl_node::CrawlNode;
// Local modules






use std::collections::{VecDeque,HashSet};
use scraper::{Html,Selector};
use url::Url;





pub async fn crawl(url:String) -> Result<HashSet<String>, reqwest::Error>{

     let mut visit_queue:VecDeque<CrawlNode> = VecDeque::new(); 

       let visited_links:HashSet<String> = HashSet::new();
        visit_queue.push_back(CrawlNode {
        url: url.to_string(),
        parent: None,
        });




      while let Some(node)=visit_queue.pop_front() {
          // if we alredy visited the link
          if visited_links.contains(&node.url){
               continue;
          }

       

         
         let html=fetch_html(&node.url).await?;
         let extracted_links = extract_links(html);
         
         for link in extracted_links{
            if !visited_links.contains(&link){
            
          
               let abs_url  =Url::parse(&link).or_else(|_|{
                  
               Url::parse(&node.url).and_then(|base| base.join(&link))

               });

               println!("abs_url {:?} ",abs_url);

               visit_queue.push_back(
                CrawlNode { 
                    url: link, 
                    parent: Some(node.url.clone())
                }
               );
            

            }
         }

         

        



          // Vist

          
      }

      println!("{:#?}",visit_queue);






// println!("body = {body:?}");
Ok(visited_links)













}



  
async fn fetch_html (url:&str)-> Result<Html,reqwest::Error>{


    let body = reqwest::get(url)
    .await?
    .text()
    .await?;
    
   let framgmnet = Html::parse_fragment(&body);

   println!("Fragments: {:?}",framgmnet);
   
   Ok(framgmnet)
   
}


pub fn extract_links(framgmnet:Html)->HashSet<String> {

 let mut links:HashSet<String> = HashSet::new();
  let selector = Selector::parse("a").unwrap();

   for link in framgmnet.select(&selector){
      if let Some(n)=link.value().attr("href"){
          links.insert(n.to_string());
         println!("link:{}",n);
      }
        
   }
  
  links

}

