

use std::sync::Arc;

use scraper::{Html, Selector};
use tokio::sync::Mutex;

  pub async fn first_page_analysis(is_first_page_analyised: &Arc<Mutex<bool>>, html_str: &str)->Option<String> {

    

     let mut analyzed: tokio::sync::MutexGuard<'_, bool> =is_first_page_analyised.lock().await;
    



    if !*analyzed{
    let html = Html::parse_document(html_str);
    

  
   
     let result = analyze_title(&html);
      *analyzed = true;
     result

  
         
    }else{
        println!("First page already analyzed");
        None
    }
  

 

    
    



    // description 

    // url 
    // canonical url
    // h1-h6
    // meta tags



    

 


    

}



pub fn analyze_title(html:&Html)->Option<String>{
    // send recommendation based on 
    // simple title tag analysis should be between 50-60 characters
    println!("Analyzing title...");

    let title_selector = Selector::parse("title").unwrap();
    if let Some(title_selector)=html.select(&title_selector).next(){

        let title_text = title_selector.text().collect::<Vec<_>>().join("");
        if title_text.len() > 60 {
            println!("Title is too long: {} characters", title_text.len());
        } else if title_text.len() < 50 {
            println!("Title is too short: {} characters", title_text.len());
        } else {
            println!("Title length is acceptable: {} characters", title_text.len());
        }

        println!("Title: {:?}", title_text);
         
    

        Some(title_text)
    }else{
        println!("No title found");
    

        None
    }
}