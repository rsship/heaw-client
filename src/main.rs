use crossterm::terminal::{Clear, ClearType};
use crossterm::Command;
use crossterm::{
    cursor,
    style::{self, Color, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::io::{self, StdoutLock, Write};

const X_SYMBOLS: &'static str = "-------";
const Y_SYMBOL: &'static str = "|";

struct DrawableEmoji<'a> {
    content: &'a str,
    x: i32,
    y: i32,
}

trait Draw {
    fn draw(emoji: DrawableEmoji, stdout: &mut StdoutLock, color: Color);
}

impl Draw for DrawableEmoji<'_> {
    fn draw<'a>(emoji: DrawableEmoji<'a>, stdout: &mut StdoutLock, color: Color) {
        let arr = emoji.content.split("\n").collect::<Vec<_>>();

        for n in 0..arr.len() {
            todo!("NOT IMPLEMENTED YET");
        }
    }
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(ClearType::All))?;

    for y in 0..40 {
        for x in 0..150 {
            if x == 0 || x == 150 - 1 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent("|".white()))?;
            } else if y == 0 || y == 40 - 1 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent("-".white()))?;
            }

            //note: middle of the screen;

            if y == 20 && x == 75 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent(X_SYMBOLS.white()))?
                    .queue(cursor::MoveTo(x, y - 1))?
                    .queue(style::PrintStyledContent(Y_SYMBOL.white()))?
                    .queue(cursor::MoveTo(x, y - 2))?
                    .queue(style::PrintStyledContent(Y_SYMBOL.white()))?
                    .queue(cursor::MoveTo(x, y - 3))?
                    .queue(style::PrintStyledContent(Y_SYMBOL.white()))?
                    .queue(cursor::MoveTo(x, y - 3))?
                    .queue(style::PrintStyledContent(Y_SYMBOL.white()))?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}
