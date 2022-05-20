use jc_dictionary_cli::{Accent, Lookup, Mode};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;
use tokio;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
        let mut lookup = Lookup::new();
        lookup.get(&to_search, Mode::NoVoice).await;
    } else {
        let mut rl = Editor::<()>::new();
        let mut lookup = Lookup::new();
        loop {
            let readline = rl.readline("~ ");
            match readline {
                Ok(input) => {
                    if input.is_empty() {
                        continue;
                    } else if input == "1" {
                        lookup.play(Accent::Uk).await;
                    } else if input == "2" {
                        lookup.play(Accent::Us).await;
                    } else if input == "i" {
                        match rl.history().last() {
                            Some(previous_search) => {
                                println!("{}", previous_search);
                                lookup.get(previous_search, Mode::Youdao).await
                            }
                            None => {
                                rl.add_history_entry(input.as_str());
                                lookup.get(&input, Mode::Default).await;
                            }
                        }
                    } else {
                        rl.add_history_entry(input.as_str());
                        lookup.get(&input, Mode::Default).await;
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
        "CLI 简单粗暴实用小词典 (v{})
        
快速搜索: 
    jc [搜索内容]

互动模式: 
    jc

在互动模式中搜索后可发送:
    '1' 播放英式发音
    '2' 播放美式发音
    'i' 在有道词典搜索

USAGE:
    jc [OPTIONS]

OPTIONS:
    -h --help     Show help message
    -v --version  Show version


————词典来源————
线下词典: ECDICT
线上词典: 有道词典
人声发音: Cambridge Dictionary
    ",
        VERSION
    );
}
