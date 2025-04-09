use std::path;
use std::fs::read_to_string;
use std::env::Args;
use regex::Regex;

pub struct ArticleArgsParams {
    pub paths: String,
    pub num: usize,
}

impl ArticleArgsParams {
     pub fn handle_article(location: ArticleArgsParams) -> Vec<String> {
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

pub fn get_args_params(mut arg: Args) ->(String, usize) {
    arg.next();
    let path = arg.next().unwrap();
    let num = arg.next()
        .expect("没有获取到字符参数")
        .parse::<usize>()
        .expect("你没有输入正确的数字");
    (path, num)
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
}