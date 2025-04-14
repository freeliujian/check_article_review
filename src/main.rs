use std::env;
use check_article_review::{get_args_params,LoopRequestSearchEngine, ArticleArgsParams, SearchEngineer, get_request_search_engineer};
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
    LoopRequestSearchEngine::loop_request_search(LoopRequestSearchEngine {
        content: article_content_vec,
        url: baidu_http
    }).await;
}
