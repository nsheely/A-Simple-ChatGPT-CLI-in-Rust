use crate::chat::{chat, Message};
use reqwest::Client;
use std::io::{Stdin, BufRead};

/*
   The `interaction.rs` module contains the functions for interacting with the user in
   different modes. It includes the following:

   - single_message(): Async function to send a single message from the user to
     the OpenAI API and print the response.
   - interactive(): Async function to enter a chat loop that continuously takes
     user input, sends it to the OpenAI API, and displays the response.

   This module is used by the `main.rs` module to process user input and handle
   different modes of interaction with the user.
*/


// This function implements the single message for the chat application.
pub async fn single_message(
    client: &Client,             // The reqwest client for making API requests.
    api_key: &str,               // The OpenAI API key.
    api_url: &str,               // The OpenAI API URL.
    model: &str,                 // The model to use for the API call.
    message: &str,               // The user's message as input.
    messages: &mut Vec<Message>, // The vector for storing the conversation messages.
) {
    // Add the user's message to the list of messages.
    messages.push(Message {
        role: "user".to_string(),
        content: message.to_string(),
    });

    // Call the chat function to send the message to the API and receive a response.
    match chat(client, api_key, api_url, model, &messages).await {
        // If the API call is successful, print the response and add it to the list of messages.
        Ok(response) => {
            println!("{}", response.content);
            messages.push(response);
        }
        // If there's an error with the API call, print the error message.
        Err(error) => {
            eprintln!("Error: {}", error)
        }
    }
}

// This function implements the interactive mode for the chat application.
pub async fn interactive(
    client: &Client,             // The reqwest client for making API requests.
    api_key: &str,               // The OpenAI API key.
    api_url: &str,               // The OpenAI API URL.
    model: &str,                 // The model to use for the API call.
    stdin: &Stdin,               // The standard input handle for reading user input.
    messages: &mut Vec<Message>, // The vector for storing the conversation messages.
) {
    // Enter a loop for the interactive mode.
    loop {
        // Prompt the user for input.
        print!("You: ");
        // Initialize a new string to store the user input.
        let mut input = String::new();
        // Read the user input from stdin and store it in `input`.
        stdin.lock().read_line(&mut input).unwrap();

        // Remove whitespace from the beginning and end of the input.
        let input = input.trim();
        // Exit the loop if the user types "exit".
        if input == "exit" {
            break;
        }
        // Print the ChatGPT prompt.
        print!("ChatGPT: ");
        // Call the single_message function to send the user input to the API and print the response.
        single_message(client, api_key, api_url, model, input, messages).await;
    }
}
