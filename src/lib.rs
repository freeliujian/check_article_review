use std::path;
use std::fs::read_to_string;
use std::env::Args;
use regex::Regex;
use reqwest;
use futures::future::join_all;



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

pub struct GetRequestSearchEngineer {
    pub url:SearchEngineer,
    pub search_content:String,
}

impl GetRequestSearchEngineer {
    pub async fn get(search_params: GetRequestSearchEngineer) -> Result<String, reqwest::Error>  {
        let get_url = match search_params.url {
            SearchEngineer::Baidu(url) => url + search_params.search_content.as_ref(),
            SearchEngineer::None => panic!("No Engineer!"),
        };
        let response = reqwest::get(get_url).await?;
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

pub fn get_file_end_name (file_name: &str) -> String {
    let file_path = path::Path::new(file_name);
    let extension = file_path.extension().and_then(|s| s.to_str());
    extension.unwrap().to_string()
}

pub async fn get_request_search_engineer(content: String) {
    let filter_em_regex = Regex::new(r"<em>(.*?)</em>");
    let filter_em_regex_pin:usize = filter_em_regex
        .unwrap().captures_iter(content.as_str())
        .filter_map(|captures| captures.get(1).map(|m| m.as_str().to_string()))
        .collect::<Vec<String>>()
        .len();

    println!("test is :{:?}", filter_em_regex_pin);
}

pub async fn loop_request_search(content: Vec<String>, url: SearchEngineer)  {
    let task = content.into_iter().map(
         |item| {
            let url = match &url {
                SearchEngineer::Baidu(url) => url.as_str(),
                SearchEngineer::None => panic!("No Engineer!"),
            };
            async move {
                let req =  GetRequestSearchEngineer::get(GetRequestSearchEngineer {
                    url:SearchEngineer::Baidu(url.to_owned()),
                    search_content: item,
                }).await.expect("TODO: panic message");
                println!("{:?}", req);
            }
        }
    );
    join_all(task).await;
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