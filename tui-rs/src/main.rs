use std::io;
use std::io::Read;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

struct App {
    terminal: Terminal<TermionBackend<termion::raw::RawTerminal<io::Stdout>>>,

    h: u16,
}

impl App {
    fn new() -> App {

        let stdout = io::stdout().into_raw_mode().unwrap();
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        App {
            terminal,

            h: 10,
        }
    }

    fn clear(&mut self) -> io::Result<()> {
        self.terminal.clear()
    }

    fn draw(&mut self) -> io::Result<()> {
        let h = self.h;

        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                //.margin(1)
                .constraints(
                    [
                    Constraint::Percentage(h),
                    Constraint::Percentage(100 - h),
                    //Constraint::Percentage(10)
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
            b'j' => {
                app.h = u16::min(90, app.h + 10);
            }
            b'k' => {
                app.h = u16::max(10, app.h - 10);
                //app.h -= 1;
            }
            _ => { }
        }
    }

    app.clear().unwrap();

}
