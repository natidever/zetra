use scraper::{Html, Selector};

async fn test_r(url:String) -> Result<(), reqwest::Error>{
    let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;
   
   let framgmnet = Html::parse_fragment(&body);
   let selector = Selector::parse("a").unwrap();

     for link in framgmnet.select(&selector){
        if let Some(n)=link.value().attr("href"){

          println!("links:{:?}",n)

        }
           
        
   }

// println!("body = {body:?}");
Ok(())


}

#[tokio::main]
async fn main (){

   if let Err(e) = test_r("url".to_string()).await {
     eprintln!("{}",e);
   }


}