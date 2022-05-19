mod ecdict;
use colored::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Suggestion {
    pub word: String,
    pub definition: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LookupResult {
    pub uk_phonetic: Option<String>,
    pub us_phonetic: Option<String>,
    pub definition: Option<String>,
    pub suggestions: Option<Vec<Suggestion>>,
}
#[derive(Clone)]
pub struct LookupError {
    pub message: String,
}

pub struct Lookup {
    ecdict: ecdict::Ecdict,
    result: Option<Result<LookupResult, LookupError>>,
}

impl Lookup {
    pub fn new() -> Self {
        Self {
            ecdict: ecdict::Ecdict::new(),
            result: None,
        }
    }

    fn print_result(self) {
        let result = self.result.unwrap();
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
                println!("{}\n", error.message);
            }
        }
    }

    pub async fn get(mut self, to_search: &str) {
        let ecdict_result = self.ecdict.lookup(to_search);

        match ecdict_result {
            Ok(ecdict_result) => match ecdict_result {
                Some(lookup_result) => self.result = Some(Ok(lookup_result)),
                None => {
                    self.result = Some(Ok(LookupResult {
                        uk_phonetic: None,
                        us_phonetic: None,
                        definition: None,
                        suggestions: None,
                    }))
                }
            },
            Err(e) => self.result = Some(Err(e)),
        }
    }
}
