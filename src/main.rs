use check_article_review::{get_request_search_engineer};
use tokio;

#[tokio::main]
async fn main() {
    // let args = env::args();
    // let (path, num) = get_args_params(args);
    // let article_content_vec = ArticleArgsParams::handle_article(
    //     ArticleArgsParams{
    //         paths: path,
    //         num
    //     }
    // );
    get_request_search_engineer().await;
}
