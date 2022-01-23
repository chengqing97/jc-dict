use colored::*;
use regex::Regex;
use reqwest::get;
use rodio::{source::Source, Decoder, OutputStream};
use std::io::Cursor;
use urlencoding::encode;

pub struct Lookup {
    pub keyword: Option<String>,
    pub definition: Option<String>,
    pub phonetics: Option<String>,
}

impl Lookup {
    pub fn new() -> Lookup {
        Lookup {
            keyword: None,
            definition: None,
            phonetics: None,
        }
    }
    pub async fn search(keyword: &str) -> Self {
        let url = String::from("https://dict.youdao.com/w/") + &encode(keyword);

        let res = get(&url).await.unwrap();
        let body = res.text().await.unwrap();

        let is_translation = is_translation(&body);
        let definition = get_definition(&body, is_translation);
        let phonetics = get_phonetics(&body, is_translation);

        match &phonetics {
            Some(value) => println!("{}", value.cyan()),
            None => (),
        }
        match &definition {
            Some(value) => println!("{}", value),
            None => println!("{}", "No result".red()),
        }

        Lookup {
            keyword: Some(String::from(keyword)),
            definition,
            phonetics,
            // is_translation: Some(is_translation),
            // uk_pronunciation: None,
            // us_pronunciation: None,
        }
    }

    pub async fn play_uk_pronunciation(&self) {
        if let Some(text) = &self.keyword {
            play_pronunciation(text, Accent::Uk).await;
        }
    }

    pub async fn play_us_pronunciation(&self) {
        if let Some(text) = &self.keyword {
            play_pronunciation(text, Accent::Us).await;
        }
    }
}

fn is_translation(body: &str) -> bool {
    let is_translate = Regex::new(r#"<div id="fanyiToggle">"#)
        .unwrap()
        .is_match(body);
    is_translate
}
fn get_phonetics(body: &str, is_translation: bool) -> Option<String> {
    if is_translation {
        return None;
    }

    let reg = Regex::new(r#"<span class="phonetic">[\s\S]*?</span>"#).unwrap();
    let mut results = vec![];
    for item in reg.captures_iter(body) {
        results.push(
            item.get(0)
                .expect("Regex for phonetics error")
                .as_str()
                .replace(r#"<span class="phonetic">"#, "")
                .replace("</span>", ""),
        );
    }
    if results.len() == 2 {
        Some(format!("英 {}  美 {}", results[0], results[1]))
    } else {
        None
    }
}
fn get_definition(body: &str, is_translation: bool) -> Option<String> {
    if is_translation {
        let meaning_paragraph = Regex::new(r#"<div class="trans-container">[\s\S]*?</div>"#)
            .unwrap()
            .find(body)
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
        Some(results[1].clone())
    } else {
        let is_found = Regex::new(r#"<h2 class="wordbook-js">"#)
            .unwrap()
            .is_match(body);
        if is_found {
            let meaning_paragraph = Regex::new(r#"<ul>[\s\S]*?</ul>"#)
                .unwrap()
                .find(body)
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
            Some(results.join("\n"))
        } else {
            None
        }
    }
}

#[derive(PartialEq)]
enum Accent {
    Uk,
    Us,
}
async fn play_pronunciation(keyword: &str, accent: Accent) -> () {
    let url = String::from("https://dictionary.cambridge.org/dictionary/english/") + keyword;
    let res = get(&url).await.unwrap();
    let body = res.text().await.unwrap();

    let reg = if accent == Accent::Uk {
        r#"media/english/uk_pron/[\S]*.mp3"#
    } else {
        r#"media/english/us_pron/[\S]*.mp3"#
    };

    let mp3_path = Regex::new(reg)
        .unwrap()
        .find(&body)
        .map(|x| x.as_str())
        .unwrap();

    let mp3_url = String::from("https://dictionary.cambridge.org/") + mp3_path;
    // println!("mp3 url: {}", mp3_url);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let resp = get(mp3_url).await.unwrap();
    let cursor = Cursor::new(resp.bytes().await.unwrap()); // Adds Read and Seek to the bytes via Cursor
    let source = Decoder::new(cursor).unwrap(); // Decoder requires it's source to impl both Read and Seek
    stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(2));
}
