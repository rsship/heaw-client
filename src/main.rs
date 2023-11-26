use anyhow::{self, Context};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, queue};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};

const USER_PROFILE: &str = "▶";
const MARGIN_X: u16 = 10;
const MARGIN_Y: u16 = 10;

struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

fn chat_area(
    stdout: &mut impl Write,
    chats: &[String],
    boundry: Rect,
) -> Result<(), std::io::Error> {
    let n = chats.len();
    let m = n.checked_sub(boundry.h as usize).unwrap_or(0);

    for (dy, chat) in chats.iter().skip(m).enumerate() {
        queue!(stdout, cursor::MoveTo(boundry.x, boundry.y + dy as u16))?;
        stdout.write(format!("{} {}", USER_PROFILE, chat).as_bytes())?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout();

    enable_raw_mode()?;
    queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
    let (mut w, mut h) = terminal::size().context("could not get size of terminal")?;

    let mut dash = "━".repeat(w as usize);
    let mut prompt = String::new();

    let chat_window_height = (h - 2) as usize;

    let mut chats = Vec::with_capacity(chat_window_height);

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
                        break 'task_looper;
                    }
                    KeyCode::Backspace => {
                        prompt.pop();
                    }
                    KeyCode::Enter => {
                        if prompt.len() > 0 {
                            chats.push(prompt.clone());
                            prompt.clear();
                        }
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

        chat_area(
            &mut stdout,
            &chats,
            Rect {
                x: MARGIN_X,
                y: 0,
                w,
                h: h - 2,
            },
        )?;
        queue!(stdout, cursor::MoveTo(0, h - 2))?;
        stdout.write(dash.as_bytes())?;

        queue!(stdout, cursor::MoveTo(MARGIN_X, h - 1))?;
        stdout.write(prompt.as_bytes())?;

        stdout.flush().context("could not flush to screen")?;
        thread::sleep(Duration::from_millis(33));
    }

    disable_raw_mode()?;

    Ok(())
}
