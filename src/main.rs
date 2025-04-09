use check_article_review::{get_args_params,ArticleArgsParams};
use std::env;
fn main() {
    let args = env::args();
    let (path, num) = get_args_params(args);
    let article_content_vec = ArticleArgsParams::handle_article(
        ArticleArgsParams{
            paths: path,
            num
        }
    );
}
