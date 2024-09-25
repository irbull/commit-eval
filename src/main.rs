use commit_eval::evaluator::evaluate_commit;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let commit_message = get_commit_message();
    let diff = get_diff();
    let result = evaluate_commit(&commit_message, &diff).await.unwrap();
    println!("{result:?}");
}

fn get_diff() -> String {
    let output = std::process::Command::new("git")
        .args(vec!["diff", "HEAD^", "HEAD"])
        .output()
        .expect("failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}

fn get_commit_message() -> String {
    let output = std::process::Command::new("git")
        .args(vec!["log", "--format=%B", "-n", "1", "HEAD"])
        .output()
        .expect("failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}
