use std::io;
use std::io::{stdin, stdout, Read, Write};
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

fn main() {

    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.clear().unwrap();

    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut bytes = stdin.bytes();

    loop {

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                //.margin(1)
                .constraints(
                    [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)
                    ].as_ref()
                ).split(f.size());

            let block = Block::default()
                .title("Block")
                .borders(Borders::ALL);

            f.render_widget(block, chunks[0]);

            let block = Block::default()
                .title("Block 2")
                .borders(Borders::ALL);

            f.render_widget(block, chunks[1]);
        }).unwrap();

        let b = bytes.next().unwrap().unwrap();
        match b {
            b'q' => {
                break;
            }
            _ => { }
        }
    }

}
