use regex::Regex;
use reqwest;

#[derive(Debug)]
pub enum Status {
  Success,
  NoResult,
}

#[derive(Debug, PartialEq)]
pub struct Suggestion {
  pub word: String,
  pub definition: String,
}

#[derive(Debug, PartialEq)]
pub struct LookupResult {
  pub uk_pronunciation: Option<String>,
  pub us_pronunciation: Option<String>,
  pub definition: Option<String>,
  pub suggestions: Option<Vec<Suggestion>>,
}

pub async fn lookup(to_search: &str) -> std::result::Result<LookupResult, reqwest::Error> {
  let url = format!("https://dict.youdao.com/w/{}", to_search);
  let response = reqwest::get(url).await?.text().await?;

  let is_translate = Regex::new(r#"<div id="fanyiToggle">"#)
    .unwrap()
    .is_match(&response);

  if is_translate {
    let definition = grab_translation(&response);
    return Ok(LookupResult {
      uk_pronunciation: None,
      us_pronunciation: None,
      definition: definition,
      suggestions: None,
    });
  }

  let is_chinese = Regex::new("英语怎么说").unwrap().is_match(&response);

  if is_chinese {
    let definition = grab_english_words(&response);
    return Ok(LookupResult {
      uk_pronunciation: None,
      us_pronunciation: None,
      definition: definition,
      suggestions: None,
    });
  }

  let has_suggestion = Regex::new("您要找的是不是").unwrap().is_match(&response);
  let has_result = Regex::new(r#"<h2 class="wordbook-js">"#)
    .unwrap()
    .is_match(&response);

  let suggestions = if has_suggestion {
    grab_suggestion(&response)
  } else {
    None
  };
  let definition = if has_result {
    grab_formal_definition(&response)
  } else {
    None
  };
  let uk_pronunciation = grab_uk_pronunciation(&response);
  let us_pronunciation = grab_us_pronunciation(&response);

  Ok(LookupResult {
    uk_pronunciation,
    us_pronunciation,
    definition,
    suggestions,
  })
}

fn grab_translation(response: &str) -> Option<String> {
  let definition_part = Regex::new(r#"(<div class="trans-container">)[\s\S]*?(</div>)"#)
    .unwrap()
    .find(response)
    .map(|x| x.as_str());

  match definition_part {
    None => return None,
    Some(definition_part) => {
      let definition = Regex::new(r#"<p>[\s\S]*?</p>"#)
        .unwrap()
        .find_iter(&definition_part)
        .map(|x| x.as_str())
        .collect::<Vec<&str>>()[1]
        .replace("<p>", "")
        .replace("</p>", "");
      return Some(definition);
    }
  }
}

fn grab_english_words(response: &str) -> Option<String> {
  let definition_part = Regex::new(r#"(<div class="trans-container">)[\s\S]*?(</div>)"#)
    .unwrap()
    .find(response)
    .map(|x| x.as_str());

  match definition_part {
    None => return None,
    Some(definition_part) => {
      let definition_raw = Regex::new(r#"(<p class="wordGroup">)[\s\S]*?(</p>)"#)
        .unwrap()
        .find_iter(&definition_part)
        .map(|x| {
          x.as_str()
            .replace(r#"<p class="wordGroup">"#, "")
            .replace("</p>", "")
        })
        .collect::<Vec<String>>();

      let mut definition_lines = vec![];
      for item in definition_raw {
        let word_type = Regex::new(r#"(;">)[\s\S]*?(</span>)"#)
          .unwrap()
          .find(&item)
          .map(|x| x.as_str().replace(r#";">"#, "").replace("</span>", ""));
        let definition = Regex::new(r#"(E2Ctranslation">)[\s\S]*?(</a>)"#)
          .unwrap()
          .find(&item)
          .map(|x| {
            x.as_str()
              .replace(r#"E2Ctranslation">"#, "")
              .replace("</a>", "")
          });

        let mut definition_line = String::new();
        if let Some(word_type) = word_type {
          if !word_type.contains(";") {
            definition_line.push_str(&word_type);
            definition_line.push_str(" ");
          }
        }
        if let Some(definition) = definition {
          definition_line.push_str(&definition);
        }
        if definition_line.len() > 0 {
          definition_lines.push(definition_line);
        }
      }
      if definition_lines.len() > 0 {
        return Some(definition_lines.join("/n"));
      } else {
        return None;
      }
    }
  }
}

fn grab_formal_definition(response: &str) -> Option<String> {
  let definition_part = Regex::new(r#"(<div class="trans-container">\s*<ul>)[\s\S]*?(</ul>)"#)
    .unwrap()
    .find(response)
    .map(|x| x.as_str());

  if definition_part == None {
    return None;
  }

  let definition = Regex::new(r#"(<li>)[\s\S]*?(</li>)"#)
    .unwrap()
    .find_iter(&definition_part.unwrap())
    .map(|x| x.as_str().replace("<li>", "").replace("</li>", ""))
    .collect::<Vec<String>>()
    .join("\n");

  Some(definition)
}

fn grab_uk_pronunciation(response: &str) -> Option<String> {
  Regex::new(r#"(<span class="pronounce">英[\s\S]*<span class="phonetic">)[\s\S]*?(</span>)"#)
    .unwrap()
    .find(response)
    .map(|x| {
      Regex::new(r#"<span class="pronounce">英[\s\S]*<span class="phonetic">|</span>"#)
        .unwrap()
        .replace_all(x.as_str(), "")
        .into_owned()
    })
}
fn grab_us_pronunciation(response: &str) -> Option<String> {
  Regex::new(r#"(<span class="pronounce">美[\s\S]*<span class="phonetic">)[\s\S]*?(</span>)"#)
    .unwrap()
    .find(response)
    .map(|x| {
      Regex::new(r#"<span class="pronounce">美[\s\S]*<span class="phonetic">|</span>"#)
        .unwrap()
        .replace_all(x.as_str(), "")
        .into_owned()
    })
}

fn grab_suggestion(response: &str) -> Option<Vec<Suggestion>> {
  let suggestion_raw = Regex::new(r#"(<p class="typo-rel">)[\s\S]*?(</p>)"#)
    .unwrap()
    .find_iter(response)
    .map(|x| {
      x.as_str()
        .replace(r#"<p class="typo-rel">"#, "")
        .replace("</p>", "")
    })
    .collect::<Vec<String>>();

  if suggestion_raw.len() == 0 {
    return None;
  }

  let mut suggestions = vec![];

  for item in suggestion_raw {
    let word = Regex::new(r#"(class="search-js">)[\s\S]*?(</a></span>)"#)
      .unwrap()
      .find(&item)
      .map(|x| {
        x.as_str()
          .replace(r#"class="search-js">"#, "")
          .replace("</a></span>", "")
      });
    let definition = Regex::new(r#"(</span>)[\s\S]*"#)
      .unwrap()
      .find(&item)
      .map(|x| x.as_str().replace(r#"</span>"#, ""))
      .unwrap_or_else(|| String::from(""))
      .trim()
      .to_owned();
    if let Some(word) = word {
      suggestions.push(Suggestion {
        word: word,
        definition,
      })
    }
  }

  if suggestions.len() == 0 {
    return None;
  }

  Some(suggestions)
}

pub struct Voice {
  pub uk: Option<String>,
  pub us: Option<String>,
}

impl Voice {
  pub fn new() -> Self {
    Self { uk: None, us: None }
  }
  pub async fn get_url(self, keyword: &str) -> std::result::Result<Self, reqwest::Error> {
    let url = format!(
      "https://dictionary.cambridge.org/dictionary/english/{}",
      keyword
    );
    let response = reqwest::get(url).await?.text().await?;

    let uk_path = Regex::new(r#"media/english/uk_pron[\S]*.ogg"#)
      .unwrap()
      .find(&response)
      .map(|x| x.as_str());
    let us_path = Regex::new(r#"media/english/us_pron[\S]*.ogg"#)
      .unwrap()
      .find(&response)
      .map(|x| x.as_str());

    let host = String::from("https://dictionary.cambridge.org/");

    Ok(Self {
      uk: if uk_path == None {
        None
      } else {
        Some(host.to_owned() + uk_path.unwrap())
      },
      us: if uk_path == None {
        None
      } else {
        Some(host.to_owned() + us_path.unwrap())
      },
    })
  }
}
