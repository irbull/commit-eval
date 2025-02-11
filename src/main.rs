use clap::Parser;
use colored::*;
use commit_eval::{evaluator::evaluate_file_change, git::{get_commit_message, get_diff, get_structured_diff}};
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
    
    println!("{}", "Changed files:".blue().bold());
    let commit_message = get_commit_message(&args.repo);
    for file in get_structured_diff(&args.repo) {
        println!("  {}", file.display().to_string().cyan());
        let diff = get_diff(&args.repo);
        let absolute_path = args.repo.join(&file);
        let file_contents = std::fs::read_to_string(&absolute_path).unwrap();
        let file_path = file.display().to_string();
        let result = evaluate_file_change(&commit_message, &file_path, &file_contents, &diff).await.unwrap();
        println!("    {}: {}", "Score".bright_blue(), format!("{}", result.score).yellow());
        println!("    {}: {}", "Reason".bright_blue(), result.reason.as_str().yellow());
    }
    println!();


}
