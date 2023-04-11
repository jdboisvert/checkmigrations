# Getting Started

## Prerequisites
- Install Rust and Cargo (https://www.rust-lang.org/tools/install)

## Building
1. Clone the repository
2. Run `cargo build` to build the project
3. Run `cargo run` to run the project (ex: `cargo run -- django ./path/to/project`)


## Testing
- Run `cargo test` to run the tests

## Formatting
- Run `cargo fmt` to format the code

## To help with local testing
### Django Project testing
It is suggested to create a `migrations` directory and to prefill if with files such as 0001_initial.py and other files to help with testing (ex: `touch migrations/0001_initial.py`). This will help with testing the detection of migrations for Django and the directory should already be ignored by git (please do not commit this directory).