# Contributing to OpenLlama
Thank you for considering contributing to OpenLlama, an open-source AI assistant for education and research. We appreciate your help in making OpenLlama a better tool for everyone.

## Setup Steps
To start contributing, follow these steps:
1. Fork the OpenLlama repository to your GitHub account.
2. Clone the forked repository to your local machine: `git clone https://github.com/your-username/OpenLlama.git`.
3. Change into the project directory: `cd OpenLlama`.
4. Install the required dependencies: `pip install -r requirements.txt`.
5. Set up a new branch for your changes (see [Branch Naming](#branch-naming)).

## Branch Naming
We use the following branch naming conventions:
* `feat/` for new features
* `fix/` for bug fixes
* `docs/` for documentation changes
* `refactor/` for code refactoring
* `test/` for test additions or changes
* `chore/` for miscellaneous changes

## Conventional Commits
We follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification for commit messages. This means that commit messages should be in the format:
`type(scope): brief description`

Where `type` is one of:
* `feat` for new features
* `fix` for bug fixes
* `docs` for documentation changes
* `refactor` for code refactoring
* `test` for test additions or changes
* `chore` for miscellaneous changes

## PR Checklist
Before submitting a pull request, make sure to:
* [ ] Run the tests: `cargo test` (Rust) or `pytest` (Python)
* [ ] Check the code style: `cargo fmt` (Rust) or `black` (Python)
* [ ] Verify that the code is consistent with the existing codebase
* [ ] Ensure that the changes are well-documented
* [ ] Include a brief description of the changes in the commit message

## Code Style
We use the following code styles:
* Rust: We follow the [Rust style guide](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) and use `cargo fmt` to format the code.
* TensorFlow: We follow the [TensorFlow style guide](https://www.tensorflow.org/community/style) and use `tf.compat.v1` for compatibility.
* PyTorch: We follow the [PyTorch style guide](https://pytorch.org/docs/stable/notes/style_guide.html) and use type hints for clarity.

## Running Tests
To run the tests, use the following commands:
* Rust: `cargo test`
* Python: `pytest`

## Reporting Bugs
To report a bug, please open a new issue with the following information:
* A brief description of the bug
* Steps to reproduce the bug
* Expected behavior
* Actual behavior
* Any relevant error messages or logs

Thank you for your contributions to OpenLlama!