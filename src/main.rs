use clap::Parser;
use commit_eval::evaluator::evaluate_commit;
use commit_eval::git::{get_commit_message, get_diff};
use conventional_commits::{parse_commit, Lexer};
use std::path::PathBuf;

/// A tool to evaluate git commit messages
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the git repository
    #[arg(short, long, default_value = ".")]
    repo: PathBuf,
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let args = Args::parse();
    
    let commit_message = get_commit_message(&args.repo);
    let diff = get_diff(&args.repo);
    let mut lexer = Lexer::new(commit_message.clone());
    let tokens = lexer.lex().ok();
    let conventional_commit = parse_commit(tokens.unwrap_or_default()).ok();
    let result = evaluate_commit(&commit_message, &diff, conventional_commit)
        .await
        .unwrap();
    println!("{result}");
}
