use std::thread;
use std::time::Duration;
use std::{convert::Infallible, io::prelude::*};

use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    //let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    // loop {
    //     let (socket, _) = listener.accept().await.unwrap();
    //     tokio::spawn(async move {
    //         handle_connection(socket).await;
    //     });
    // }

    let sample_irc_message = "@id=234AB :dan!d@localhost PRIVMSG #chan :Hey what's up!\r\n";
    tokenize_irc_message(&sample_irc_message);
}

type MessageBuffer = [u8; 512 + 4096];

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer: MessageBuffer = [0; 512 + 4096];
    let size = stream.write(&buffer).await.unwrap();

    let buffer_as_str = std::str::from_utf8(&buffer).unwrap();

    // let mut start: usize = 0; let mut end: usize = 0;
    // while start < size {
    //     let c = buffer_as_str.chars()[start];
    //     match c {
    //         '@' => println!("Tag Token"),
    //         ":" => println!("Source Token"),
    //         _ => println!("Command or Parameter Token"),
    //     }
    // }
    //    let response = "GREETINGS FROM THE SERVER\r\n";
    //    stream.write(response.as_bytes()).await.unwrap();
    //
    //    return stream.flush().await.unwrap();
}

enum IrcMessageTokenizerState {
    StartOfData,
    TokenizeTags,
    EndOfTags,
    TokenizeSource,
    EndOfSource,
    TokenizeCommand,
    TokenizeParameters,
    EndOfData,
}

fn tokenize_irc_message(message: &str) {
    let mut tokenizer_state = IrcMessageTokenizerState::StartOfData;
    let mut message_iterator = message.chars();

    while let Some(character) = message_iterator.next() {
        match (&tokenizer_state, &character) {
            (IrcMessageTokenizerState::StartOfData, '@') => {
                println!("Tokenize Tag Start!");
                tokenizer_state = IrcMessageTokenizerState::TokenizeTags
            }
            (IrcMessageTokenizerState::StartOfData, ':') => {
                println!("Tokenize Source Start!");
                tokenizer_state = IrcMessageTokenizerState::TokenizeSource
            }
            (IrcMessageTokenizerState::StartOfData, _) => {
                println!("Tokenize Command Start!");
                tokenizer_state = IrcMessageTokenizerState::TokenizeCommand
            }
            (IrcMessageTokenizerState::TokenizeTags, ' ') => {
                println!("Tokenize Tag End!");
                tokenizer_state = IrcMessageTokenizerState::EndOfTags
            }
            (IrcMessageTokenizerState::TokenizeTags, _) => {
                println!("Tokenize Tag: {}", character);
            }
            (IrcMessageTokenizerState::EndOfTags, ':') => {
                println!("Tokenize Tag End!");
                tokenizer_state = IrcMessageTokenizerState::TokenizeSource
            }
            (IrcMessageTokenizerState::EndOfTags, ' ') => {
                println!("Ignoring Whitespace");
            }
            (IrcMessageTokenizerState::EndOfTags, _) => {
                tokenizer_state = IrcMessageTokenizerState::TokenizeCommand
            }
            (IrcMessageTokenizerState::TokenizeSource, ' ') => {
                println!("Tokenize Source End!");
                tokenizer_state = IrcMessageTokenizerState::EndOfSource
            }
            (IrcMessageTokenizerState::TokenizeSource, _) => {
                println!("Tokenize Source: {}", character);
            }
            (IrcMessageTokenizerState::EndOfSource, ' ') => {
                println!("Ignoring Whitespace");
            }
            (IrcMessageTokenizerState::EndOfSource, _) => {
                tokenizer_state = IrcMessageTokenizerState::TokenizeCommand
            }
            (IrcMessageTokenizerState::TokenizeCommand, ' ') => {
                println!("Tokenize Command End!");
                println!("Tokenize Paramter Start!");
                tokenizer_state = IrcMessageTokenizerState::TokenizeParameters
            }
            (IrcMessageTokenizerState::TokenizeCommand, _) => {
                println!("Tokenize Command: {}", character);
            }
            (IrcMessageTokenizerState::TokenizeParameters, ' ') => {
                println!("Tokenize Parameter Continue!");
                tokenizer_state = IrcMessageTokenizerState::TokenizeParameters
            }
            (IrcMessageTokenizerState::TokenizeParameters, '\r') => {
                println!("Tokenize Parameter End!");
                tokenizer_state = IrcMessageTokenizerState::EndOfData
            }
            (IrcMessageTokenizerState::TokenizeParameters, '\n') => {
                println!("Tokenize Parameter End!");
                tokenizer_state = IrcMessageTokenizerState::EndOfData
            }
            (IrcMessageTokenizerState::TokenizeParameters, _) => {
                println!("Tokenize Parameter: {}", character);
            }
            (_, _) => println!("Uh oh!"),
        }
    }
}
