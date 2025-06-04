

// our will take a url it it will return Error or the html 
async fn test_r(url:String) -> Result<(), reqwest::Error>{
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



// 