use clap::{App, Arg};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{self, BufRead};

/*
   This ChatGPT Rust CLI program allows users to interact with OpenAI's ChatGPT using
   the command-line interface.

   The main function initializes the necessary configuration, such as the API key,
   API URL, and the chat model to use. It then checks for command-line arguments
   and decides the appropriate mode to run:

   1. Non-Interactive Single Message Mode: If a message is provided as a command-line
      argument, the program sends this single message to ChatGPT and displays the response.
   2. Non-Interactive Stdin Message Mode: If no arguments are provided, the program
      reads a single message from stdin, sends it to ChatGPT, and displays the response.
   3. Interactive Mode: If started with the --interactive flag, the program enters a chat loop
      that continuously takes user input, sends it to ChatGPT, and displays the response. The loop
      continues until the user enters "exit."

   To communicate with ChatGPT, the program uses the `chat` function, which sends an HTTP request
   to the API and processes the response. Error handling is implemented using the `CustomError` enum.
*/

// Custom error type to handle both request and parsing errors.
#[derive(Debug)]
enum CustomError {
    ReqwestError(reqwest::Error), // Represents an error from the reqwest library.
    ParseError(String),           // Represents a JSON parsing error with a custom message.
}

// Implement the Display trait for CustomError to provide a user-readable error message.
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::ReqwestError(e) => write!(f, "Reqwest error: {}", e), // Format the ReqwestError variant.
            CustomError::ParseError(s) => write!(f, "Parse error: {}", s), // Format the ParseError variant.
        }
    }
}

// Implement the From trait to convert a reqwest::Error into a CustomError.
impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> CustomError {
        CustomError::ReqwestError(err) // Wrap the reqwest::Error into the ReqwestError variant.
    }
}

// Structure for serializing the chat request to the OpenAI API.
#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,          // The model to use for the API call.
    messages: &'a [Message], // The slice containing the conversation messages.
}

// Structure for storing a message within the conversation.
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Message {
    role: String,    // The role of the message's author ("user", "assistant", or "system").
    content: String, // The content of the message.
}

// Structure for deserializing the chat response from the OpenAI API.
#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>, // A vector of choices (responses) returned by the API.
}

// Structure for deserializing the individual choice in the chat response.
#[derive(Deserialize)]
struct Choice {
    message: Message, // The message contained within the choice.
}

// This function sends a chat request to the OpenAI API and receives the response.
async fn chat(
    client: &Client,      // The reqwest client for making API requests.
    api_key: &str,        // The OpenAI API key.
    api_url: &str,        // The OpenAI API URL.
    model: &str,          // The model to use for the API call.
    messages: &[Message], // The slice containing the conversation messages.
) -> Result<Message, CustomError> {
    // Create a ChatRequest object using the provided model and messages.
    let request = ChatRequest { model, messages };

    // Send the chat request to the OpenAI API.
    let response = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key)) // Add the API key to the request headers.
        .json(&request) // Serialize the request object as JSON.
        .send() // Send the request.
        .await?; // Await the response.

    // Get the text content of the response.
    let response_text = response.text().await?;

    // Try to deserialize the response text into a ChatResponse object.
    let chat_response: Result<ChatResponse, _> = serde_json::from_str(&response_text);
    if let Ok(chat_response) = chat_response {
        // If deserialization is successful, extract the first choice from the response.
        if let Some(choice) = chat_response.choices.into_iter().next() {
            // Return the message from the extracted choice.
            Ok(choice.message)
        } else {
            // If there are no choices, return a default message.
            Ok(Message {
                role: "assistant".to_string(),
                content: "I don't have an answer for that.".to_string(),
            })
        }
    } else {
        // If deserialization fails, print the raw response and return a custom error.
        println!("Raw response: {}", response_text);
        Err(CustomError::ParseError(
            "Error parsing the API response".to_string(),
        ))
    }
}

// This function implements the single message mode for the chat application.
async fn single_message_mode(
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
async fn interactive_mode(
    client: &Client,             // The request client for making API requests.
    api_key: &str,               // The OpenAI API key.
    api_url: &str,               // The OpenAI API URL.
    model: &str,                 // The model to use for the API call.
    stdin: &std::io::Stdin,      // The standard input handle for reading user input.
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
        // Call the single_message_mode function to send the user input to the API and print the response.
        single_message_mode(client, api_key, api_url, model, input, messages).await;
    }
}

// The main function is an asynchronous function as it calls other async functions.
#[tokio::main]
async fn main() {
    // Define the command-line interface using clap.
    let matches = App::new("ChatGPT")
        .version("1.0")
        .about("Interact with OpenAI's ChatGPT")
        .arg(
            // Define a command-line argument for single-message mode.
            Arg::with_name("input")
                .help("Provide input message for single-message mode")
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
        single_message_mode(
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
        interactive_mode(&client, &api_key, api_url, model, &stdin, &mut messages).await;
    } else {
        // If no argument or flag, use non-interactive mode: single message from stdin.
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        single_message_mode(
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
