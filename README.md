# Commit Message Evaluation Tool

This is a Rust project for evaluating Git commit messages and their corresponding diffs. The tool leverages OpenAI's GPT model to determine if a commit message is complete, adequate, or needs revision, providing suggestions for revisable messages.

## Features

- Parse and analyze Git commit messages and diffs.
- Evaluate commit messages using a trained AI model.
- Generate suggestions for improving commit messages.
- Strict JSON schema validation for evaluation responses.

## Dependencies

- Rust (latest stable version)
- [serde](https://docs.rs/serde/latest/serde/)
- [serde_json](https://docs.rs/serde_json/latest/serde_json/)
- [schemars](https://docs.rs/schemars/latest/schemars/)
- [async-openai](https://docs.rs/async-openai/latest/async_openai/)
- [askama](https://docs.rs/askama/latest/askama/)
- [conventional_commits](https://github.com/shubhexists/conventional_commits/) 

## Usage

1. Clone the repository:
    ```sh
    git clone https://github.com/irbull/commit-eval
    cd commit-eval
    ```

2. Ensure you have an `.env` file set up with necessary environment variables, especially for OpenAI API keys.

3. Run the project using Cargo:
    ```sh
    cargo run
    ```

## Example

When you run the project, it fetches the latest commit message and its diff from the current Git repository and evaluates it. An example output might look like:

```
🟨 Revisable
Reason: The commit message is too vague.
---
fix: minor bug fix

Fixed an issue with the user login process by addressing a null pointer exception.
```

## Modules

### `Evaluation`

Contains definitions and implementations for `MessageScore` and `Evaluation`. These are used to represent and display the evaluation results.

### `Evaluator`

Handles the interaction with the OpenAI API to evaluate the commit messages and diffs.

### `Git`

Provides functions to fetch the latest commit message and its diff from the Git repository.

### Filters

Custom filters used within Askama templates to handle optional values.

## Function Documentation

- `evaluate_commit(message: &str, diff: &str, conventional_commit: Option<ConventionalCommit>) -> Result<Evaluation, Box<dyn Error>>`
  Evaluates a commit message and its diff, returning an `Evaluation` object.

- `construct_system_prompt(conventional_commit: Option<ConventionalCommit>) -> String`
  Constructs the system prompt for the AI model based on the commit type.

- `construct_user_message(message: &str, diff: &str) -> String`
  Constructs the user message for the AI model based on the commit message and diff.

- `get_diff() -> String`
  Retrieves the diff of the current `HEAD` commit.

- `get_commit_message() -> String`
  Retrieves the commit message of the current `HEAD` commit.

## Contributing

Feel free to submit issues and pull requests. For major changes, please open an issue first to discuss what you would like to change.

## Acknowledgements

- OpenAI for providing the GPT model.
- Authors of the crates used in this project.

---

Happy committing and code reviewing! 🚀
