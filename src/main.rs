use commit_eval::evaluator::evaluate_commit;
use commit_eval::git::{get_commit_message, get_diff};
use conventional_commits::{parse_commit, Lexer};

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let commit_message = get_commit_message();
    let diff = get_diff();
    let mut lexer = Lexer::new(commit_message.clone());
    let tokens = lexer.lex().ok();
    let conventional_commit = parse_commit(tokens.unwrap_or_default()).ok();
    let result = evaluate_commit(&commit_message, &diff, conventional_commit)
        .await
        .unwrap();
    println!("{result}");
}
