use regex::Regex;
use reqwest;

#[derive(Debug)]
pub enum Status {
  Success,
  NoResult,
}

#[derive(PartialEq)]
pub enum Accent {
  Uk,
  Us,
}

#[derive(Debug)]
pub struct Suggestion {
  pub word: String,
  pub definition: String,
}

#[derive(Debug)]
pub struct LookupResult {
  pub uk_pronunciation: Option<String>,
  pub us_pronunciation: Option<String>,
  pub definition: Option<String>,
  pub status: Status,
  pub suggestions: Option<Vec<Suggestion>>,
}

impl LookupResult {
  fn new(status: Status) -> Self {
    LookupResult {
      uk_pronunciation: None,
      us_pronunciation: None,
      definition: None,
      status: status,
      suggestions: None,
    }
  }
  fn no_result() -> Self {
    Self::new(Status::NoResult)
  }
  fn definition(definition: &str) -> Self {
    LookupResult {
      uk_pronunciation: None,
      us_pronunciation: None,
      definition: Some(String::from(definition)),
      status: Status::Success,
      suggestions: None,
    }
  }
}

pub async fn lookup(to_search: &str) -> std::result::Result<LookupResult, reqwest::Error> {
  let url = format!("https://dict.youdao.com/w/{}", to_search);
  let response = reqwest::get(url).await?.text().await?;

  let is_translate = Regex::new(r#"<div id="fanyiToggle">"#)
    .unwrap()
    .is_match(&response);

  if is_translate {
    return Ok(grab_translation(&response));
  }

  let is_chinese = Regex::new("英语怎么说").unwrap().is_match(&response);

  if is_chinese {
    return Ok(grab_english_words(&response));
  }

  let has_suggestion = Regex::new("您要找的是不是").unwrap().is_match(&response);

  if has_suggestion {
    return Ok(grab_suggestion(&response));
  }

  let has_result = Regex::new(r#"<h2 class="wordbook-js">"#)
    .unwrap()
    .is_match(&response);
  if has_result {
    return Ok(grab_formal_definition(&response));
  }

  Ok(LookupResult::no_result())
}

fn grab_translation(response: &str) -> LookupResult {
  let definition_part = Regex::new(r#"(<div class="trans-container">)[\s\S]*?(</div>)"#)
    .unwrap()
    .find(response)
    .map(|x| x.as_str());

  match definition_part {
    None => return LookupResult::no_result(),
    Some(definition_part) => {
      let definition = Regex::new(r#"<p>[\s\S]*?</p>"#)
        .unwrap()
        .find_iter(&definition_part)
        .map(|x| x.as_str())
        .collect::<Vec<&str>>()[1]
        .replace("<p>", "")
        .replace("</p>", "");
      return LookupResult::definition(&definition);
    }
  }
}

fn grab_english_words(response: &str) -> LookupResult {
  let definition_part = Regex::new(r#"(<div class="trans-container">)[\s\S]*?(</div>)"#)
    .unwrap()
    .find(response)
    .map(|x| x.as_str());

  match definition_part {
    None => return LookupResult::no_result(),
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
        return LookupResult::definition(&definition_lines.join("/n"));
      } else {
        return LookupResult::no_result();
      }
    }
  }
}

fn grab_formal_definition(response: &str) -> LookupResult {
  let definition_part = Regex::new(r#"(<div class="trans-container">\s*<ul>)[\s\S]*?(</ul>)"#)
    .unwrap()
    .find(response)
    .map(|x| x.as_str());

  if definition_part == None {
    return LookupResult::no_result();
  }

  let definition = Regex::new(r#"(<li>)[\s\S]*?(</li>)"#)
    .unwrap()
    .find_iter(&definition_part.unwrap())
    .map(|x| x.as_str().replace("<li>", "").replace("</li>", ""))
    .collect::<Vec<String>>()
    .join("\n");

  let uk_pronunciation =
    Regex::new(r#"(<span class="pronounce">英[\s\S]*<span class="phonetic">)[\s\S]*?(</span>)"#)
      .unwrap()
      .find(response)
      .map(|x| {
        Regex::new(r#"<span class="pronounce">英[\s\S]*<span class="phonetic">|</span>"#)
          .unwrap()
          .replace_all(x.as_str(), "")
          .into_owned()
      });
  let us_pronunciation =
    Regex::new(r#"(<span class="pronounce">美[\s\S]*<span class="phonetic">)[\s\S]*?(</span>)"#)
      .unwrap()
      .find(response)
      .map(|x| {
        Regex::new(r#"<span class="pronounce">美[\s\S]*<span class="phonetic">|</span>"#)
          .unwrap()
          .replace_all(x.as_str(), "")
          .into_owned()
      });

  LookupResult {
    uk_pronunciation,
    us_pronunciation,
    definition: Some(definition),
    status: Status::Success,
    suggestions: None,
  }
}

fn grab_suggestion(response: &str) -> LookupResult {
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
    return LookupResult::no_result();
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
    return LookupResult::no_result();
  }
  LookupResult {
    uk_pronunciation: None,
    us_pronunciation: None,
    definition: None,
    status: Status::Success,
    suggestions: Some(suggestions),
  }
}

pub async fn get_pronunciation_url(
  keyword: &str,
  accent: Accent,
) -> std::result::Result<Option<String>, reqwest::Error> {
  let url = format!(
    "https://dictionary.cambridge.org/dictionary/english/{}",
    keyword
  );
  let response = reqwest::get(url).await?.text().await?;

  let reg = if accent == Accent::Uk {
    r#"media/english/uk_pron/[\S]*.mp3"#
  } else {
    r#"media/english/us_pron/[\S]*.mp3"#
  };

  let mp3_path = Regex::new(reg).unwrap().find(&response).map(|x| x.as_str());

  if mp3_path == None {
    return Ok(None);
  }

  let mp3_url = String::from("https://dictionary.cambridge.org/") + mp3_path.unwrap();

  Ok(Some(mp3_url))
}
