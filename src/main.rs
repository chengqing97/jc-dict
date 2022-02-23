use colored::*;
use reqwest;
use rodio::{source::Source, Decoder, OutputStream};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;
use std::io::Cursor;
use std::result::Result;
use tokio;
use wd_dict::{get_pronunciation_url, lookup, Accent, Status};

const VERSION: &str = "0.2.0";

async fn process(to_search: &str) {
  let result = lookup(&to_search).await;
  match result {
    Ok(result) => match result.status {
      Status::Success => {
        if let Some(definition) = result.definition {
          let mut pronunciation = String::new();
          if let Some(uk_pronunciation) = result.uk_pronunciation {
            pronunciation.push_str(&format!("英 {}  ", &uk_pronunciation));
          }
          if let Some(us_pronunciation) = result.us_pronunciation {
            pronunciation.push_str(&format!("美 {}", &us_pronunciation));
          }
          if pronunciation.len() > 0 {
            println!("{}", pronunciation.cyan())
          }
          println!("{}", definition)
        } else if let Some(suggestions) = result.suggestions {
          println!("Are you looking for");
          for item in suggestions {
            println!("{}: {}", item.word.magenta(), item.definition)
          }
        } else {
          println!("No result")
        }
      }
      Status::NoResult => {
        println!("No result")
      }
    },
    Err(error) => {
      println!("{}", error)
    }
  }
}

async fn play_pronunciation(url: &str) -> Result<(), reqwest::Error> {
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  let response = reqwest::get(url).await.unwrap();
  let cursor = Cursor::new(response.bytes().await.unwrap()); // Adds Read and Seek to the bytes via Cursor
  let source = Decoder::new(cursor).unwrap(); // Decoder requires it's source to impl both Read and Seek
  let _play = stream_handle.play_raw(source.convert_samples());
  std::thread::sleep(std::time::Duration::from_secs(2));
  Ok(())
}

#[tokio::main]
async fn main() {
  let mut args: Vec<String> = env::args().collect();

  if args.contains(&String::from("-h")) | args.contains(&String::from("--help")) {
    print_help();
  } else if args.contains(&String::from("-v")) | args.contains(&String::from("--version")) {
    println!("Version: {}", VERSION);
  } else if args.len() > 1 {
    args.remove(0);
    let to_search = args.join(" ");
    process(&to_search).await;
  } else {
    let mut rl = Editor::<()>::new();
    loop {
      let readline = rl.readline("~ ");
      match readline {
        Ok(input) => {
          if input.is_empty() {
            continue;
          } else if input == "1" || input == "2" {
            let last_keyword = rl.history().last();
            if last_keyword == None {
              println!("Search something first and then send '1' or '2' to play pronunciation\n")
            }
            let url = get_pronunciation_url(
              last_keyword.unwrap(),
              if input == "1" { Accent::Uk } else { Accent::Us },
            )
            .await;
            match url {
              Ok(url) => match url {
                Some(url) => {
                  play_pronunciation(&url)
                    .await
                    .unwrap_or_else(|e| println!("{}", e));
                }
                None => println!(
                  "{}",
                  "No pronunciation playback found for this word\n".red()
                ),
              },
              Err(error) => println!("{}\n", error),
            }
          } else {
            rl.add_history_entry(input.as_str());
            process(&input).await;
            println!("");
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
    "
CLI 有道词典 (v{})

Interactive mode: 
wd

Quick search: 
wd word you want to search

Flags:
-h --help     Show help message
-v --version  Show version
    ",
    VERSION
  );
}
