use anyhow::{self, Context};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, execute, queue, QueueableCommand};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};

const USER_PROFILE: &str = "▶";

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout();

    enable_raw_mode()?;
    queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
    let (mut w, mut h) = terminal::size().context("could not get size of terminal")?;

    let mut dash = "━".repeat(w as usize);
    let mut prompt = String::new();

    let mut chats = Vec::new();

    'task_looper: loop {
        while poll(Duration::ZERO)? {
            match read()? {
                Event::Resize(width, height) => {
                    w = width;
                    h = height;
                    dash = "━".repeat(w as usize);
                }
                Event::Key(event) => match event.code {
                    KeyCode::Esc => {
                        println!("{:?}", event.code);
                        break 'task_looper;
                    }
                    KeyCode::Backspace => {
                        prompt.pop();
                    }
                    KeyCode::Enter => {
                        chats.push(prompt.clone());
                        prompt.clear();
                    }
                    KeyCode::Char(x) => {
                        prompt.push(x);
                    }
                    _ => {}
                },
                _ => {}
            };
        }

        queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
        let mut y = 0;
        for chat in chats.iter() {
            queue!(stdout, cursor::MoveTo(0, y))?;
            stdout.write(format!("{}    {}", USER_PROFILE, chat).as_bytes())?;
            y += 2;
        }

        queue!(stdout, cursor::MoveTo(0, h - 2))?;
        stdout.write(dash.as_bytes())?;

        queue!(stdout, cursor::MoveTo(0, h - 1))?;
        stdout.write(prompt.as_bytes())?;

        stdout.flush().context("could not flush to screen")?;
        thread::sleep(Duration::from_millis(33));
    }

    disable_raw_mode()?;

    Ok(())
}
