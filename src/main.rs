use std::{env, process};
use check_article_review::{get_args_params,LoopRequestSearchEngine, ArticleArgsParams, SearchEngineer};
use tokio;



#[tokio::main]
async fn main() {
    let args = env::args();
    let args_params  = get_args_params(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });
    let article_content_vec = ArticleArgsParams::handle_article(
        ArticleArgsParams{
            paths: args_params.paths,
            num:args_params.num
        }
    );
    let baidu_http = SearchEngineer::new(String::from("baidu"));
    LoopRequestSearchEngine::loop_request_search(LoopRequestSearchEngine {
        content: article_content_vec,
        url: baidu_http
    }).await;
}

