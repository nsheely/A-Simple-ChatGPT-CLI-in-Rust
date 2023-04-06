# A Simple ChatGPT Rust CLI

This project is a simple command-line interface (CLI) for interacting with OpenAI's ChatGPT made using the Rust programming language. 

The application supports passing messages to ChatGPT via stdin and taking a single message as an argument.

Additionally it has an interactive mode, where you can have a ongoing conversation until you enter 'quit' or 'exit'.

## Features

- Send messages from stdin
- Send single message to ChatGPT
- Interactive chat loop with ChatGPT

## Dependencies

This project uses the following crates:

- `tokio`: Asynchronous runtime
- `reqwest`: HTTP client for API calls
- `serde_json`: JSON parsing
- `clap`: Command-line argument parsing

## Usage

Build the project:
```bash
cargo build --release
```
Set your OpenAI API key:
```bash
export OPENAI_API_KEY="your_api_key_here"
```
You can get a key at https://platform.openai.com/

Run the application in interactive mode:
```bash
./target/release/chatgpt-rust-cli --interactive
```
Send a single message:
```bash
./target/release/chatgpt-rust-cli "What is the capital of France?"
```
Send a message from stdin:
```bash
echo "What is the capital of France?" | ./target/release/chatgpt-rust-cli
```
