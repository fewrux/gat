use std::env::args;
use std::process::{exit, Command};

/// Executes a Git command with the given arguments.
///
/// # Arguments
///
/// * `command`: A string representing the Git command to execute.
/// * `args`: A slice of strings representing the arguments for the Git command.
fn execute_git_command(command: &str, args: &[&str]) {
    let git_command = Command::new("git")
        .arg(command)
        .args(args)
        .output()
        .expect(&format!("Failed to execute git {} command", command));

    if !git_command.status.success() {
        eprintln!("Error: Failed to {} changes.", command);
        exit(1);
    }
}

/// Main function that reads the commit message from command-line arguments,
/// adds, commits, and pushes changes to the Git repository.
fn main() {
    execute_git_command("add", &["-A"]);

    let commit_message = args().nth(1).unwrap_or_else(|| {
        eprintln!("Error: No commit message provided.");
        exit(1);
    });

    if commit_message.trim().is_empty() {
        eprintln!("Error: Commit message cannot be empty.");
        exit(1);
    }

    const MAX_COMMIT_MESSAGE_LENGTH: usize = 100;
    if commit_message.len() > MAX_COMMIT_MESSAGE_LENGTH {
        eprintln!(
            "Error: Commit message is too long. Please limit it to {} characters.",
            MAX_COMMIT_MESSAGE_LENGTH
        );
        exit(1);
    }

    execute_git_command("commit", &["-m", &commit_message]);

    const GIT_BRANCH: &str = "main";
    execute_git_command("push", &["origin", GIT_BRANCH]);

    println!("Successfully added, committed, and pushed all changes!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_git_command_success() {
        // Test a successful Git command execution
        execute_git_command("add", &["file.txt"]);
        // Assert that the test passed (no panic occurred)
        assert!(true);
    }

    #[test]
    #[should_panic]
    fn test_execute_git_command_failure() {
        // Test a failing Git command execution
        execute_git_command("invalid-command", &[]);
    }
}
