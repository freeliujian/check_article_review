use std::env;
use check_article_review::{get_args_params, loop_request_search, ArticleArgsParams, SearchEngineer};
use tokio;



#[tokio::main]
async fn main() {
    let args = env::args();
    let (path, num) = get_args_params(args);
    let article_content_vec = ArticleArgsParams::handle_article(
        ArticleArgsParams{
            paths: path,
            num
        }
    );
    let baidu_http = SearchEngineer::new(String::from("baidu"));
    loop_request_search(article_content_vec,baidu_http).await;
}
