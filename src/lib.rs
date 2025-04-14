use std::cell::RefCell;
use std::path;
use std::fs::read_to_string;
use std::env::Args;
use std::rc::Rc;
use regex::Regex;
use futures::future::join_all;
use reqwest::{get, Error};
use scraper::{ Selector, Html};

pub struct ArticleArgsParams {
    pub paths: String,
    pub num: usize,
}

impl ArticleArgsParams {
    pub fn handle_article(location: ArticleArgsParams) -> Vec<String> {
        if location.num > 32  {
            panic!("Invalid article number: {}", location.num);
        }
        let path = path::Path::new(location.paths.as_str());
        let regx = Regex::new(r"[\r\n]+").unwrap();
        let num = location.num;
        let article_content = read_to_string(path).unwrap();
        let article_content_format = article_content
            .trim()
            .replace("\n\r", "")
            .replace("\n", "")
            .replace("\r", "")
            .replace(' ', "");
        let regx_content = regx.replace_all(&article_content_format, "")
            .chars()
            .collect::<Vec<char>>()
            .chunks(num)
            .map(|chunk| {
                chunk.iter().collect::<String>()
            })
            .collect::<Vec<String>>();
        regx_content
    }
}
#[derive(Debug)]
pub struct GetRequestSearchEngineer {
    pub url:SearchEngineer,
    pub search_content:String,
}

impl GetRequestSearchEngineer {
    pub async fn get(url:String) -> Result<String, Error>  {
        let response = get(url).await?;
        response.text().await
    }
}

#[derive(Debug)]
pub enum SearchEngineer{
    Baidu(String),
    None
}

impl SearchEngineer{

    pub fn new(text: String) -> Self {
        let mut search_engineer:SearchEngineer =SearchEngineer::None;
        let mut add_search_end = String::new();
        if text.to_string() == "baidu".to_string() {
            add_search_end = String::from("s?wd=");
            let baidu_url_http = String::from("http://www.baidu.com/");
            search_engineer = SearchEngineer::Baidu(baidu_url_http + &add_search_end)
        }
        search_engineer
    }
}


pub fn get_args_params(mut arg: Args) ->(String, usize) {
    arg.next();
    let path = arg.next().unwrap();
    let num = arg.next()
        .expect("没有获取到字符参数")
        .parse::<usize>()
        .expect("你没有输入正确的数字");
    (path, num)
}

fn get_file_end_name (file_name: &str) -> String {
    let file_path = path::Path::new(file_name);
    let extension = file_path.extension().and_then(|s| s.to_str());
    extension.unwrap().to_string()
}

pub async fn get_request_search_engineer(content: String) -> Option<usize> {
    let mut filter_em_regex_pin:usize =1;
    let document = Html::parse_document(content.as_str());
    let selector = Selector::parse("div#content_left").unwrap();
    if let Some(div) = document.select(&selector).next() {
        let left_div = div.inner_html();
        let filter_em_regex = Regex::new(r"<em>(.*?)</em>");
        filter_em_regex_pin = filter_em_regex
            .unwrap().captures_iter(left_div.as_str())
            .filter_map(|captures| captures.get(1).map(|m| m.as_str().to_string()))
            .collect::<Vec<String>>()
            .len();
    };
    Some(filter_em_regex_pin)
}

pub struct LoopRequestSearchEngine {
    pub content: Vec<String>,
    pub url:SearchEngineer,
}
impl LoopRequestSearchEngine {
    pub async fn loop_request_search(params: LoopRequestSearchEngine) {
        let mut items = Rc::new(RefCell::new(Vec::new()));
        let urls = params.content.into_iter().map({
            let items = Rc::clone(&items);
            move |item| {
                items.borrow_mut().push(item.clone());
                let get_url = match &params.url {
                    SearchEngineer::Baidu(url) => {
                        let new_url = format!("{}{}", url,item);
                        new_url
                    },
                    SearchEngineer::None => panic!("No Engineer!"),
                };
                GetRequestSearchEngineer::get(get_url)
            }
        }).collect::<Vec<_>>();

        let results = join_all(urls).await;
        for (i, result) in results.into_iter().enumerate() {
             match result {
                Ok(msg) => {
                   let get_pin =  get_request_search_engineer(msg).await;
                    println!("当前文本：{}， 在搜索引擎提及频率是： {:?}",items.borrow()[i], get_pin.expect("获取错误信息"));
                },
                Err(e) => eprintln!("[{}] Error: {}", i, e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_location_for_file() {

    }

    #[test]
    fn test_get_arg_params() {

    }

    #[test]
    fn test_get_file_end_name() {
        get_file_end_name("readme.md");
        assert_eq!("md", get_file_end_name("./../../README.md"));
    }

    #[test]
    fn test_get_request_search_engineer() {
    }
}