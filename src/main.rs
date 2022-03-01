use colored::*;
use reqwest;
use rodio::{source::Source, Decoder, OutputStream};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;
use std::io::Cursor;
use std::result::Result;
use tokio;
use wd_dict::{lookup, LookupResult, Voice};

const VERSION: &str = "0.3.1";

// To make color work on Windows:
// reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1
// https://stackoverflow.com/questions/51680709/colored-text-output-in-powershell-console-using-ansi-vt100-codes

async fn process(to_search: &str) -> std::result::Result<LookupResult, reqwest::Error> {
  let result = lookup(&to_search).await;
  match result {
    Ok(ref result) => {
      let mut pronunciation = String::new();
      if let Some(ref uk_pronunciation) = result.uk_pronunciation {
        pronunciation.push_str(&format!("英 {}  ", &uk_pronunciation));
      }
      if let Some(ref us_pronunciation) = result.us_pronunciation {
        pronunciation.push_str(&format!("美 {}", &us_pronunciation));
      }
      if pronunciation.len() > 0 {
        println!("{}", pronunciation.cyan());
      }
      if let Some(ref definition) = result.definition {
        println!("{}\n", definition);
      }

      if let Some(ref suggestions) = result.suggestions {
        println!("Are you looking for");
        for item in suggestions {
          println!("{}: {}", item.word.magenta(), item.definition);
        }
        println!();
      }

      if result.suggestions == None
        && result.definition == None
        && result.uk_pronunciation == None
        && result.us_pronunciation == None
      {
        println!("{}\n", "No result".red());
      }
    }
    Err(ref error) => {
      println!("{}\n", error);
    }
  }
  result
}

async fn play_pronunciation(url: &str) -> Result<(), reqwest::Error> {
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
    let _result = process(&to_search).await;
  } else {
    let mut rl = Editor::<()>::new();
    let mut voice = Voice::new();
    let mut is_first_search = true;
    loop {
      let readline = rl.readline("~ ");
      match readline {
        Ok(input) => {
          if input.is_empty() {
            continue;
          } else if input == "1" || input == "2" {
            if is_first_search {
              println!("Search something first and then send '1' or '2' to play pronunciation\n");
            } else if voice.uk != None && input == "1" {
              play_pronunciation(&voice.uk.as_ref().unwrap())
                .await
                .unwrap_or_else(|e| println!("{}", e));
            } else if voice.us != None && input == "2" {
              play_pronunciation(&voice.us.as_ref().unwrap())
                .await
                .unwrap_or_else(|e| println!("{}", e));
            } else {
              println!(
                "{}",
                "No pronunciation playback found for this word\n".red()
              );
            }
          } else {
            rl.add_history_entry(input.as_str());
            is_first_search = false;
            let result = process(&input).await;
            if let Ok(_) = result {
              let voice_result = Voice::new().get_url(&input).await;
              if let Ok(voice_result) = voice_result {
                voice = voice_result;
              }
            }
          }
        }
        Err(ReadlineError::Interrupted) => {
          break;
        }
        Err(error) => {
          println!("Error: {:?}\n", error);
        }
      }
    }
  }
}

fn print_help() {
  println!(
    "CLI 有道词典 (v{})

Interactive mode: 
wd

Quick search: 
wd word you want to search

Send '1' or '2' after searching something in interactive mode to play pronunciation 

Flags:
-h --help     Show help message
-v --version  Show version
    ",
    VERSION
  );
}
