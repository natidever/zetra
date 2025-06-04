
use tokio;
use reqwest;
use scraper::{Html,Selector };


#[tokio::main]
async fn main() {
//  let fun_test =   test_r().await;

//  if let Err(e)= fun_test{
//     eprintln!("Error:{}",e)
//  }
 
 
}



async fn test_r() -> Result<(), reqwest::Error>{
    let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;
   
   let framgmnet = Html::parse_fragment(&body);
   let selector = Selector::parse("a").unwrap();

   for link in framgmnet.select(&selector){
        println!("links:{:?}",link)
   }

// println!("body = {body:?}");
Ok(())


}