use std::io;
use std::io::{stdin, stdout, Read, Write};
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

struct App {

    //terminal: Terminal<tui::backend::TermionBackend>
    terminal: Terminal<TermionBackend<termion::raw::RawTerminal<io::Stdout>>>,
    //terminal: Terminal<TermionBackend<io::Stdout>>,
}

impl App {
    fn new() -> App {

        let stdout = io::stdout().into_raw_mode().unwrap();
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        App {
            terminal
        }
    }

    fn clear(&mut self) -> io::Result<()> {
        self.terminal.clear()
    }

    fn draw(&mut self) -> io::Result<()> {
        self.terminal.draw(|f| {
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
        })
    }
}

fn main() {

    let mut app = App::new();

    app.clear().unwrap();

    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut bytes = stdin.bytes();

    loop {
        app.draw().unwrap();

        let b = bytes.next().unwrap().unwrap();
        match b {
            b'q' => {
                break;
            }
            _ => { }
        }
    }

}
