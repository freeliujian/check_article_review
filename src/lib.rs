//! 这是文章检测重复的核心库
//!
//! 他提供讲文章拆分成不同的str去baidu请求
//!
//! ## 使用方法
//! `cargo run -- file_path number`
//!
//!

use std::cell::RefCell;
use std::path;
use std::fs::read_to_string;
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

    /// 这是处理文本的函数，主要用来切割和去除空格
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
    /// 简单的封装了reqwest的get方法
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
    /// 拼接请求url
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

/// 获取args的函数
pub fn get_args_params(mut arg: impl Iterator<Item = String>) ->Result<ArticleArgsParams, Error> {
    arg.next();
    let path = arg.next().unwrap();
    let num = arg.next()
        .expect("没有获取到字符参数")
        .parse::<usize>()
        .expect("你没有输入正确的数字");
    Ok(ArticleArgsParams{paths: path, num})
}

fn get_file_end_name (file_name: &str) -> String {
    let file_path = path::Path::new(file_name);
    let extension = file_path.extension().and_then(|s| s.to_str());
    extension.unwrap().to_string()
}
///获取频率函数
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
    ///轮询获取频率函数
    pub async fn loop_request_search(params: LoopRequestSearchEngine) {
        let items = Rc::new(RefCell::new(Vec::new()));
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
    use std::vec::IntoIter;



    #[test]
    fn test_search_engineer() {
        let output = match SearchEngineer::new(String::from("baidu")) {
            SearchEngineer::Baidu(url) => url,
            SearchEngineer::None => panic!("No Engineer!"),
        };
        assert_eq!("https://www.baidu.com/s?wd=", output);
    }

    fn mock_args<I,T>(args: I) -> IntoIter<String>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>
    {
        let mut v = vec!["cargo run".to_string()];
        v.extend(args.into_iter().map(|a| a.into()));
        v.into_iter()
    }

    #[test]
    fn test_get_args_params() {
        let mock_vec = vec![ "./example/1.txt","10"];
        let mock_data = mock_args(mock_vec);
        let result = get_args_params(mock_data);
        assert_eq!("./example/1.txt", result.iter().clone().next().unwrap().paths);
        assert_eq!("10", result.iter().clone().next().unwrap().num.to_string());
    }

    #[test]
    fn test_get_file_end_name() {
        get_file_end_name("readme.md");
        assert_eq!("md", get_file_end_name("./../../README.md"));
    }

    #[tokio::test]
    async fn test_get_request_search_engineer() {
        let test_html = String::from(r"
        <html>
            <div id='content_left'>
                <em>test content</em>
                no em test content
            </div>
        </html>
        ");
        let output:usize = 1;
        let content_result = Rc::new(RefCell::new(None));
        let result_clone = Rc::clone(&content_result);
        async {
            let result =  get_request_search_engineer(test_html).await;
            *result_clone.borrow_mut() = result;
        }.await;
        let actual_result = content_result.borrow().unwrap();
        assert_eq!(output, actual_result);
    }
}
