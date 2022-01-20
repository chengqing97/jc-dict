use colored::*;
use regex::Regex;
use reqwest::get;
use std::env::args;
use std::io::{self, BufRead, Write};
use urlencoding::encode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keywords: Vec<String> = args().collect();
    if keywords.len() <= 1 {
        print!("~ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let to_search = &line.unwrap();
            if to_search != "" {
                search(to_search).await?;
                println!("");
                print!("~ ");
                io::stdout().flush().unwrap();
            }
        }
    } else {
        let to_search = &keywords[1..].join(" ");
        search(&to_search).await?
    }

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
    }
}
