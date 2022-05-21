use crate::Voice;
use regex::Regex;
use reqwest;

pub async fn get_playback_url(keyword: &str) -> Result<Voice, reqwest::Error> {
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

    Ok(Voice {
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
