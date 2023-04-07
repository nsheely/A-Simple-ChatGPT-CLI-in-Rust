use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt;

/*
   The `chat.rs` module contains the data structures and functions related to communication
   with the OpenAI API. It includes the following:

   - CustomError: Enum type for handling errors from the Reqwest library and JSON parsing.
   - ChatRequest: Structure for serializing the chat request to the OpenAI API.
   - Message: Structure for storing a message within the conversation.
   - ChatResponse: Structure for deserializing the chat response from the OpenAI API.
   - Choice: Structure for deserializing the individual choice in the chat response.
   - chat(): Async function to send a chat request to the OpenAI API and receive the response.

   This module is used by the `interaction.rs` module to interact with the OpenAI API.
*/


// Custom error type to handle both request and parsing errors.
#[derive(Debug)]
pub enum CustomError {
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
pub struct Message {
    pub role: String,    // The role of the message's author ("user", "assistant", or "system").
    pub content: String, // The content of the message.
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
pub async fn chat(
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
