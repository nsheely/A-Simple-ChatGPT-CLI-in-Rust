use clap::{App, Arg};
use reqwest::Client;
use std::io::{self, BufRead};

mod chat;
mod interaction;

use chat::Message;
use interaction::{interactive, single_message};

/*
   The `main.rs` module is the entry point for the ChatGPT Rust CLI program. It sets up the
   necessary configuration, such as the API key, API URL, and the chat model to use. It then
   checks for command-line arguments and decides the appropriate mode to run:

   1. Single Message: If a message is provided as a command-line
      argument, the program sends this single message to ChatGPT and displays the response.
   2. Stdin Message: If no arguments are provided, the program
      reads a single message from stdin, sends it to ChatGPT, and displays the response.
   3. Interactive: If started with the --interactive flag, the program enters a chat loop
      that continuously takes user input, sends it to ChatGPT, and displays the response. The loop
      continues until the user enters "exit."

   The `main.rs` module imports necessary structures and functions from the `chat.rs`
   and `interaction.rs` modules to communicate with the OpenAI API and process user input.
*/


// The main function is an asynchronous function as it calls other async functions.
#[tokio::main]
async fn main() {
    // Define the command-line interface using clap.
    let matches = App::new("ChatGPT")
        .version("1.0")
        .about("Interact with OpenAI's ChatGPT")
        .arg(
            // Define a command-line argument for single-message.
            Arg::with_name("input")
                .help("Provide input message for a single-message")
                .index(1),
        )
        .arg(
            // Define a command-line flag for interactive mode.
            Arg::with_name("interactive")
                .help("Enable interactive mode")
                .long("interactive")
                .short('i')
                .takes_value(false),
        )
        .get_matches();

    // Get the OpenAI API key from the environment variables.
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found");
    // Set the URL for making requests to the OpenAI API.
    let api_url = "https://api.openai.com/v1/chat/completions";
    // Set the model we want to use for the API call.
    let model = "gpt-3.5-turbo";

    // Create a new reqwest client to make requests.
    let client = Client::new();
    // Get the standard input handle for reading input.
    let stdin = io::stdin();
    // Initialize a vector to store the conversation messages.
    let mut messages: Vec<Message> = vec![Message {
        role: "system".to_string(),
        content: "You are ChatGPT, a large language model trained by OpenAI.".to_string(),
    }];

    // If there is a command-line argument, use single-message mode.
    if let Some(input) = matches.value_of("input") {
        single_message(
            &client,
            &api_key,
            api_url,
            model,
            input.trim(),
            &mut messages,
        )
        .await;
    } else if matches.is_present("interactive") {
        // If the interactive flag is present, use interactive mode.
        interactive(&client, &api_key, api_url, model, &stdin, &mut messages).await;
    } else {
        // If no argument or flag, use a single message from stdin.
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        single_message(
            &client,
            &api_key,
            api_url,
            model,
            input.trim(),
            &mut messages,
        )
        .await;
    }
}
