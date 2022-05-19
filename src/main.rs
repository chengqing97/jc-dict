use colored::*;
use reqwest;
use rodio::{source::Source, Decoder, OutputStream};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;
use std::io::Cursor;
// use std::result::Result;
use tokio;
use wd_dict::{Lookup, LookupError, LookupResult};

const VERSION: &str = "0.3.1";

// To make color work on Windows:
// reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1
// https://stackoverflow.com/questions/51680709/colored-text-output-in-powershell-console-using-ansi-vt100-codes

async fn play_phonetic(url: &str) -> Result<(), reqwest::Error> {
    // Reference: https://stackoverflow.com/questions/63463503/playing-audio-from-url-in-rust
    let (_stream, stream_handle) = OutputStream::try_default().expect("Initialize error");
    let response = reqwest::get(url).await?;
    let cursor = Cursor::new(response.bytes().await.expect("Get bytes error"));
    let source = Decoder::new(cursor).expect("Decode error");
    let _play = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_millis(1500));
    Ok(())
}

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.contains(&String::from("-h")) | args.contains(&String::from("--help")) {
        print_help();
    } else if args.contains(&String::from("-v")) | args.contains(&String::from("--version")) {
        println!("{}", VERSION);
    } else if args.len() > 1 {
        args.remove(0);
        let to_search = args.join(" ");
        let lookup = Lookup::new();
        lookup.get(&to_search).await;
    } else {
        let mut rl = Editor::<()>::new();
        let lookup = Lookup::new();
        // let mut voice = Voice::new();
        let mut is_first_search = true;
        let mut db = loop {
            let readline = rl.readline("~ ");
            match readline {
                Ok(input) => {
                    if input.is_empty() {
                        continue;
                    } else if input == "1" || input == "2" {
                        if is_first_search {
                            println!("Search something first and then send '1' or '2' to play phonetic\n");
                        } else if voice.uk != None && input == "1" {
                            play_phonetic(&voice.uk.as_ref().unwrap())
                                .await
                                .unwrap_or_else(|e| println!("{}", e));
                        } else if voice.us != None && input == "2" {
                            play_phonetic(&voice.us.as_ref().unwrap())
                                .await
                                .unwrap_or_else(|e| println!("{}", e));
                        } else {
                            println!("{}", "No phonetic playback found for this word\n".red());
                        }
                    } else {
                        rl.add_history_entry(input.as_str());
                        is_first_search = false;
                        let result = lookup.get(&input).await;
                        // if let Ok(_) = result {
                        //   let voice_result = Voice::new().get_url(&input).await;
                        //   if let Ok(voice_result) = voice_result {
                        //     voice = voice_result;
                        //   }
                        // }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    break;
                }
                Err(error) => {
                    println!("Error: {:?}\n", error);
                }
            }
        };
    }
}

fn print_help() {
    println!(
        "CLI 有道词典 (v{})

Interactive mode: 
wd

Quick search: 
wd word you want to search

Send '1' or '2' after searching something in interactive mode to play phonetic 

Flags:
-h --help     Show help message
-v --version  Show version
    ",
        VERSION
    );
}
