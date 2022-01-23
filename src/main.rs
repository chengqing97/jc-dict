use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env::args;
use std::error::Error;
use wd_dict::Lookup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut rl = Editor::<()>::new();
    let history_path = format!(
        "{}/wd-history.txt",
        dirs::document_dir().unwrap().to_str().unwrap()
    );
    rl.load_history(&history_path).ok();
    if args().len() <= 1 {
        let mut lookup = Lookup::new();
        loop {
            let readline = rl.readline("~ ");
            match readline {
                Ok(line) => match &line[..] {
                    "" => continue,
                    "1" => {
                        lookup.play_uk_pronunciation().await;
                        continue;
                    }
                    "2" => {
                        lookup.play_us_pronunciation().await;
                        continue;
                    }
                    _ => {
                        rl.add_history_entry(line.as_str());
                        lookup = Lookup::search(line.as_str()).await;
                        println!();
                    }
                },
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
        Lookup::search(keywords).await;
    }

    rl.save_history(&history_path).unwrap();

    Ok(())
}
