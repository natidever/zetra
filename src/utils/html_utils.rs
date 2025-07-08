

use std::sync::Arc;

use scraper::{Html, Selector};
use serde::de;
use serde_json::{Value,json};
use tokio::sync::Mutex;

  pub async fn first_page_analysis(is_first_page_analyised: &Arc<Mutex<bool>>, html_str: &str)->Option<Value> {

    println!("Analyzing first page...");

     let mut analyzed: tokio::sync::MutexGuard<'_, bool> =is_first_page_analyised.lock().await;
    


    if !*analyzed{
    let html = Html::parse_document(html_str);
    

  
   
     let title_recommendation = analyze_title(&html);
    //  let description_recommendation = None; // Placeholder for future description analysis
    //     let url_recommendation = None; // Placeholder for future URL analysis
    //     let canonical_url_recommendation = None; // Placeholder for future canonical URL analysis
    //     let h1_h6_recommendation = None; // Placeholder for future H1-H6 analysis
      *analyzed = true;

     Some(
         json!({
        "title_recommendation": title_recommendation,
      })
     )

     



  
         
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
    

    let title_selector = Selector::parse("title").unwrap();
    if let Some(title_selector)=html.select(&title_selector).next(){
        let recommendation;

        let title_text = title_selector.text().collect::<Vec<_>>().join("");
        if title_text.len() > 60 {
            recommendation="Title is too long: {} characters"
        } else if title_text.len() < 50 {
            recommendation="Title is too short: {} characters"
        } else {
            recommendation="Title length is acceptable: {} characters"
        }

        println!("Title: {:?}", title_text);
         
    

        Some(recommendation.to_string())
    }else{
        println!("No title found");
    

        None
    }
}