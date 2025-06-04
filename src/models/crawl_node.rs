#[derive(Debug,Clone)]
pub struct CrawlNode{
   pub url:String,
    pub parent: Option<String>,
}