mod cambridge;
mod ecdict;
mod youdao;
use colored::*;
use rodio::{source::Source, Decoder, OutputStream};
use std::io::Cursor;

#[derive(Debug, PartialEq, Clone)]
pub struct Suggestion {
    word: String,
    definition: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LookupResult {
    uk_phonetic: Option<String>,
    us_phonetic: Option<String>,
    definition: Option<String>,
    suggestions: Option<Vec<Suggestion>>,
}

#[derive(Clone)]
pub struct LookupError {
    pub message: String,
}

#[derive(PartialEq)]
pub enum Mode {
    Youdao,
    Default,
    NoVoice,
}

#[derive(PartialEq)]
pub enum Accent {
    Uk,
    Us,
}
pub struct Voice {
    pub uk: Option<String>,
    pub us: Option<String>,
}

pub struct Lookup {
    ecdict: ecdict::Ecdict,
    result: Option<Result<LookupResult, LookupError>>,
    voice: Voice,
}

impl Lookup {
    pub fn new() -> Self {
        Self {
            ecdict: ecdict::Ecdict::new(),
            result: None,
            voice: Voice { uk: None, us: None },
        }
    }

    fn print_result(&self) {
        let result = self.result.as_ref().unwrap();
        match result {
            Ok(result) => {
                let mut phonetic = String::new();
                if let Some(ref uk_phonetic) = result.uk_phonetic {
                    phonetic.push_str(&format!("英 {}  ", &uk_phonetic));
                }
                if let Some(ref us_phonetic) = result.us_phonetic {
                    phonetic.push_str(&format!("美 {}", &us_phonetic));
                }
                if phonetic.len() > 0 {
                    println!("{}", phonetic.cyan());
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
                    && result.uk_phonetic == None
                    && result.us_phonetic == None
                {
                    println!("{}\n", "No result".red());
                }
            }
            Err(ref error) => {
                self.print_error(&error.message);
                // println!("{}\n", error.message);
            }
        }
    }

    fn print_error(&self, message: &str) {
        println!("{}\n", message);
    }

    pub async fn get(&mut self, to_search: &str, mode: Mode) {
        if mode == Mode::Youdao {
            self.result = Some(youdao::get(to_search).await);
        } else {
            let ecdict_result = self.ecdict.get(to_search);

            match ecdict_result {
                Ok(ecdict_result) => match ecdict_result {
                    Some(lookup_result) => self.result = Some(Ok(lookup_result.clone())),
                    None => {
                        self.result = Some(youdao::get(to_search).await);
                    }
                },
                Err(e) => self.result = Some(Err(e.clone())),
            }
        }

        self.print_result();

        if mode != Mode::NoVoice {
            let voice_result = cambridge::get_playback_url(to_search).await;
            if let Ok(voice_result) = voice_result {
                self.voice = voice_result;
            }
        }
    }

    pub async fn play(&self, accent: Accent) {
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

        if let None = self.result {
            println!("Search something first and then send '1' or '2' to play voice\n");
        } else if self.voice.uk != None && accent == Accent::Uk {
            play_phonetic(self.voice.uk.as_ref().unwrap())
                .await
                .unwrap_or_else(|e| println!("{}", e));
        } else if self.voice.us != None && accent == Accent::Us {
            play_phonetic(self.voice.us.as_ref().unwrap())
                .await
                .unwrap_or_else(|e| println!("{}", e));
        } else {
            println!("{}", "No playback found for this word\n".red());
        }
    }
}
