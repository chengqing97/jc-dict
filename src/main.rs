use colored::*;
use regex::Regex;
use reqwest::get;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env::args;
use urlencoding::encode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = Editor::<()>::new();
    let history_path = format!(
        "{}/wd-history.txt",
        dirs::document_dir().unwrap().to_str().unwrap()
    );
    rl.load_history(&history_path).ok();
    if args().len() <= 1 {
        loop {
            let readline = rl.readline("~ ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    search(line.as_str()).await?;
                    println!();
                }
                Err(ReadlineError::Interrupted) => break, // ctrl + c
                Err(ReadlineError::Eof) => break,         // ctrl + d
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
    } else {
        let keywords: Vec<String> = args().collect();
        let keywords = &keywords[1..].join(" ");
        rl.add_history_entry(keywords);
        search(keywords).await?
    }
    rl.save_history(&history_path).unwrap();

    Ok(())
}

async fn search(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = String::from("https://dict.youdao.com/w/") + &encode(text);

    let res = get(&url).await?;
    let body = res.text().await?;
    print_pronunciation(&body);
    print_meaning(&body);

    Ok(())
}

fn print_pronunciation(text: &str) {
    let reg = Regex::new(r#"<span class="phonetic">[\s\S]*?</span>"#).unwrap();
    let mut results = vec![];

    for item in reg.captures_iter(text) {
        results.push(
            item.get(0)
                .unwrap()
                .as_str()
                .replace(r#"<span class="phonetic">"#, "")
                .replace("</span>", ""),
        );
    }

    if results.len() == 2 {
        let to_print = format!("英 {}  美 {}", results[0], results[1]);
        println!("{}", to_print.cyan());
    }
}
fn print_meaning(text: &str) {
    let is_translate = Regex::new(r#"<div id="fanyiToggle">"#)
        .unwrap()
        .is_match(text);
    if is_translate {
        let meaning_paragraph = Regex::new(r#"<div class="trans-container">[\s\S]*?</div>"#)
            .unwrap()
            .find(text)
            .map(|x| x.as_str())
            .unwrap();
        let mut results = vec![];
        let reg = Regex::new(r#"<p>[\s\S]*?</p>"#).unwrap();
        for item in reg.captures_iter(meaning_paragraph) {
            results.push(
                item.get(0)
                    .unwrap()
                    .as_str()
                    .replace("<p>", "")
                    .replace("</p>", ""),
            );
        }
        println!("{}", results[1]);
    } else {
        let is_found = Regex::new(r#"<h2 class="wordbook-js">"#)
            .unwrap()
            .is_match(text);
        if is_found {
            let meaning_paragraph = Regex::new(r#"<ul>[\s\S]*?</ul>"#)
                .unwrap()
                .find(text)
                .map(|x| x.as_str())
                .unwrap();
            let mut results = vec![];
            let reg = Regex::new(r#"<li>[\s\S]*?</li>"#).unwrap();
            for item in reg.captures_iter(meaning_paragraph) {
                results.push(
                    item.get(0)
                        .unwrap()
                        .as_str()
                        .replace("<li>", "")
                        .replace("</li>", ""),
                );
            }
            println!("{}", results.join("\n"));
        } else {
            println!("{}", "No result".red());
        }
    }
}
