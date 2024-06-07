use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
    QueueableCommand,
};
use std::io::{stdout, Write};

pub fn menu<'a, S>(label: &str, items: &'a [S]) -> &'a S
where
    S: AsRef<str> + std::fmt::Display,
{
    let _ = enable_raw_mode();
    let mut stdout = stdout();
    let len: u16 = items.len() as u16;
    let _ = stdout.queue(terminal::ScrollUp(len));
    let _ = stdout.queue(cursor::MoveUp(len));
    let mut exit = false;
    let mut position: usize = 0;
    let mut selected_position = 0;

    while !exit {
        let _ = stdout.queue(cursor::SavePosition);
        let _ = stdout.write(label.as_bytes());

        items.iter().enumerate().for_each(|(index, item)| {
            let _ = stdout.queue(cursor::MoveToColumn(0));
            let _ = stdout.queue(cursor::MoveDown(1));

            if index == position {
                let _ = stdout.write(format!("* {}", item).as_bytes());
            } else {
                let _ = stdout.write(format!("- {}", item).as_bytes());
            }
        });
        let _ = stdout.queue(cursor::RestorePosition);
        let _ = stdout.flush();

        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Esc | KeyCode::Char('q') => exit = true,
                KeyCode::Char('c') => {
                    if event.modifiers == KeyModifiers::CONTROL {
                        exit = true
                    }
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if position < items.len() - 1 {
                        position += 1
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    position = position.saturating_sub(1);
                }
                KeyCode::Enter => {
                    selected_position = position;
                    exit = true;
                }
                _ => {}
            }
        }
    }

    let _ = stdout.queue(terminal::Clear(ClearType::CurrentLine));
    let _ = stdout.flush();
    let _ = disable_raw_mode();

    items.iter().enumerate().nth(selected_position).unwrap().1
}

//
// Could not get tests working, read this blog, it will iprobably help
// https://jmmv.dev/2020/12/unit-testing-a-console-app.html
//
#[cfg(test)]
mod tests {

    #[test]
    fn test_menu() {}
}
