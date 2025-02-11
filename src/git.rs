use std::path::Path;

#[must_use]
/// Get the diff of the current HEAD.
/// # Panics
pub fn get_diff<P: AsRef<Path>>(repo_path: P) -> String {
    let output = std::process::Command::new("git")
        .current_dir(repo_path)
        .args(vec!["diff", "HEAD^", "HEAD"])
        .output()
        .expect("failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}

#[must_use]
/// Get the commit message of the current HEAD.
/// # Panics
pub fn get_commit_message<P: AsRef<Path>>(repo_path: P) -> String {
    let output = std::process::Command::new("git")
        .current_dir(repo_path)
        .args(vec!["log", "--format=%B", "-n", "1", "HEAD"])
        .output()
        .expect("failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}
