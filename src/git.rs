use std::path::{Path, PathBuf};

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
/// Get individual diffs for each file in the provided list
/// # Panics
pub fn get_file_diffs<P: AsRef<Path>>(repo_path: P, files: &[PathBuf]) -> Vec<String> {
    files
        .iter()
        .map(|file| {
            let output = std::process::Command::new("git")
                .current_dir(&repo_path)
                .args(["diff", "HEAD^", "HEAD", "--", file.to_str().unwrap()])
                .output()
                .expect("failed to execute process");

            String::from_utf8(output.stdout).unwrap()
        })
        .collect()
}

#[must_use]
/// Get a list of files that have changed in the current HEAD.
/// # Panics
pub fn get_structured_diff<P: AsRef<Path>>(repo_path: P) -> Vec<PathBuf> {
    let output = std::process::Command::new("git")
        .current_dir(repo_path)
        .args(vec!["diff", "--name-only", "HEAD^", "HEAD"])
        .output()
        .expect("failed to execute process");

    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(PathBuf::from)
        .collect()
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
