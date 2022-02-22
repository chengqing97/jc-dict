// use rustyline::error::ReadlineError;
// use rustyline::Editor;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Print;
use crossterm::terminal::ClearType;
use crossterm::{cursor, event, execute, terminal};
use std::env::args;
use std::error::Error;
use std::io::{stdout, Write};
use wd_dict::Lookup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let mut rl = Editor::<()>::new();
    // let history_path = format!(
    //     "{}/wd-history.txt",
    //     dirs::document_dir().unwrap().to_str().unwrap()
    // );
    // rl.load_history(&history_path).ok();
    if args().len() <= 1 {
        let _clean_up = CleanUp;
        terminal::enable_raw_mode()?;
        let mut line_reader = LineReader::new();
        loop {
            line_reader.run().await;
            if line_reader.should_continue == false {
                break;
            }
        }
    } else {
        let keywords: Vec<String> = args().collect();
        let to_search = &keywords[1..].join(" ");
        // rl.add_history_entry(keywords);
        Lookup::search(to_search).await;
    }

    // rl.save_history(&history_path).unwrap();

    Ok(())
}

struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
    }
}

struct LineReader {
    history: Vec<String>,
    current_line: String,
    current_history_index: Option<usize>,
    should_continue: bool,
    cursor_position: (u16, u16),
}
impl LineReader {
    fn new() -> Self {
        let cursor_position = cursor::position().unwrap();
        Self {
            history: Vec::new(),
            current_line: String::new(),
            current_history_index: None,
            should_continue: true,
            cursor_position,
        }
    }

    async fn run(&mut self) {
        execute!(stdout(), Print("~ ")).unwrap();
        loop {
            if let Event::Key(event) = event::read().unwrap() {
                match event {
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: event::KeyModifiers::CONTROL,
                    } => {
                        execute!(stdout(), Print("\n\r")).unwrap();
                        self.should_continue = false;
                        break;
                    }
                    KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: event::KeyModifiers::NONE,
                    } => {
                        self.current_line.pop();
                    }
                    KeyEvent {
                        code: KeyCode::Left,
                        modifiers: event::KeyModifiers::NONE,
                    } => {
                        // cursor::MoveLeft(1);
                        let (x, y) = cursor::position().unwrap();
                        execute!(stdout(), cursor::MoveTo(x - 1, y));
                    }
                    KeyEvent {
                        code: KeyCode::Right,
                        modifiers: event::KeyModifiers::NONE,
                    } => {
                        cursor::MoveRight(1);
                    }
                    KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: event::KeyModifiers::NONE,
                    } => {
                        execute!(stdout(), Print("\n\r")).unwrap();
                        if self.current_line != "" {
                            println!("{:?}\r", &self.current_line);
                            self.history.push(self.current_line.to_owned());
                            Lookup::search(&self.current_line).await;
                            self.current_line.clear();
                        }
                        break;
                    }
                    KeyEvent {
                        code: KeyCode::Up,
                        modifiers: event::KeyModifiers::NONE,
                    } => {
                        if self.history.len() > 0 {
                            match self.current_history_index {
                                Some(i) => {
                                    if i > 0 {
                                        self.current_history_index = Some(i - 1);
                                        self.current_line.clear();
                                        self.current_line.push_str(
                                            &self.history[self.current_history_index.unwrap()],
                                        );
                                    }
                                }
                                None => {
                                    self.current_history_index = Some(self.history.len() - 1);
                                    self.current_line.clear();
                                    self.current_line.push_str(
                                        &self.history[self.current_history_index.unwrap()],
                                    );
                                }
                            }
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Down,
                        modifiers: event::KeyModifiers::NONE,
                    } => match self.current_history_index {
                        None => {}
                        Some(i) => {
                            if i == self.history.len() - 1 {
                                self.current_line.clear();
                                self.current_history_index = None;
                            } else {
                                self.current_history_index = Some(i + 1);
                                self.current_line.clear();
                                self.current_line
                                    .push_str(&self.history[self.current_history_index.unwrap()]);
                            }
                        }
                    },
                    KeyEvent {
                        code: KeyCode::Char(c),
                        modifiers: event::KeyModifiers::NONE,
                    } => self.current_line.push(c),
                    _ => {}
                }
            }
            self.rerender();
        }
    }

    fn rerender(&self) {
        execute!(
            stdout(),
            terminal::Clear(ClearType::CurrentLine),
            Print("\r"),
            Print("~ "),
            Print(&self.current_line)
        )
        .unwrap();
    }
}
