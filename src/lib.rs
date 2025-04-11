use std::path;
use std::fs::read_to_string;
use std::env::Args;
use regex::Regex;
use reqwest;



pub struct ArticleArgsParams {
    pub paths: String,
    pub num: usize,
}

pub struct GetRequestSearchEngineer {
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
            .replace("\r", "");
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

impl GetRequestSearchEngineer {
    pub async fn get(url: &str) -> Result<String, reqwest::Error>  {
        let response = reqwest::get(url).await?;
        response.text().await
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

pub async fn get_request_search_engineer()  {
    let test = match GetRequestSearchEngineer::get("http://www.baidu.com/s?wd=%E6%88%91%E6%9C%89%E4%B8%80%E4%B8%AA%E6%A2%A6%E6%83%B3").await {
        Ok(res) => res,
        Err(e) => panic!("{}", e),
    };
    let filter_em_regex = Regex::new(r"<em\b[^>]*>(.*?)</em>");
    let filter_em_regex_pin = filter_em_regex.unwrap().replace_all(&test, "").to_string();
    println!("test is :{:?}", test);
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