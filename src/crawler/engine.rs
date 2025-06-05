
#![allow(unused_variables)]

use crate::models::crawl_node::CrawlNode;
// Local modules

use std::collections::{VecDeque,HashSet};
use scraper::{Html,Selector};
use url::Url;
use std::fs;
use std::time::Instant;






pub async fn crawl(url:String) -> Result<HashSet<String>, reqwest::Error>{
    
    let start =Instant::now();

     let base_url= Url::parse(&url).unwrap();

     let mut visit_queue:VecDeque<CrawlNode> = VecDeque::new(); 

       let visited_links:HashSet<String> = HashSet::new();
        visit_queue.push_back(CrawlNode {
        url: url.to_string(),
        parent: None,
        });


      let mut file_index = 0;

      while let Some(node)=visit_queue.pop_front() {
          // if we alredy visited the link

          if visited_links.contains(&node.url){
               continue;
          }

       

         
         let (raw_html,raw_string)=fetch_html(&node.url).await?;
         // creating html file 
         println!("Visiting \u{1F310} {} ", {&node.url});
         let file_name = format!("page_{}.html",file_index);

         let file_object=fs::write(file_name.to_string(), raw_string).expect("Unable to write");
         file_index +=1;
         // 

          
         let extracted_links = extract_links(raw_html);
         
         for link in extracted_links{
            if !visited_links.contains(&link){
            
          
               let abs_url  =Url::parse(&link).or_else(|_|{
                  
               Url::parse(&node.url).and_then(|base| base.join(&link))

               });


               // Avoid crawling external websites 


               if base_url.domain()!=abs_url.as_ref().unwrap().domain(){

                  // println!("ExternalURL:{:?}",abs_url);
                  continue;
               }
               // println!("abs_url {:?} ",abs_url.as_ref().unwrap().to_string());

               visit_queue.push_back(
                CrawlNode { 
                    url: abs_url.unwrap().to_string(), 
                    parent: Some(node.url.clone())
                }
               );
            

            }
         }
          
          if file_index == 5 {
            break;
          }
         

        



          // Vist

          
      }

      // println!("{:#?}",visit_queue);






// println!("body = {body:?}");
println!("Total Elapased Time {:?}",start.elapsed());

Ok(visited_links)













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

