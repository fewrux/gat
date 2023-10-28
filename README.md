# gat - Git Automation Tool

This Rust application automates the process of adding, committing, and pushing changes to a Git repository. It simplifies the workflow by handling these tasks automatically, allowing users to focus on their code changes.

### Table of Contents

1. [gat - Git Automation](#gat---git-automation)
2. [Features](#features)
3. [Prerequisites](#prerequisites)
4. [Installation](#installation)
    - [Clone the Repository](#clone-the-repository)
    - [Navigate to the Project Directory](#navigate-to-the-project-directory)
    - [Build in Release Mode](#build-in-release-mode)
    - [Install the Application](#install-the-application)
5. [Usage](#usage)
6. [Customizing Branch Name](#customizing-branch-name)
7. [Upcoming Features](#upcoming-features)


## Features

- **Simplified Workflow**: Automates the Git commands (`git add -A`, `git commit -m`, and `git push origin main`) to streamline the process.
- **Error Handling**: Provides informative error messages if any Git command fails, making it easy to identify and fix issues.
- **Customizable Branch**: Allows configuration of the target branch for pushing changes (default: `main`).

## Prerequisites

- [Rust](https://www.rust-lang.org/): Make sure you have Rust installed on your system.

## Installation

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/fewrux/gat.git
   ```

2. **Navigate to the Project Directory:**
   ```bash
   cd gat
   ```

3. **Build in release mode:**
   ```bash
   cargo build --release
   ```

4. **Install the application:**
   ```bash
   cargo install --path .
   ```

## Usage

Provide your commit message as a command-line argument:

   ```bash
   gat "Your commit message here"
   ```
Replace "Your commit message here" with your actual commit message.

## Customizing Branch Name

By default, the application pushes changes to the main branch. If you want to push changes to a different branch, modify the GIT_BRANCH constant in the main.rs file.

  ```bash
  const GIT_BRANCH: &str = "main";
  ```

Replace "main" with the name of your desired branch.

## Upcoming Features

- [ ] Branch customization through command-line argument
