use std::io;
use std::io::Read;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use tui::widgets::canvas::Canvas;
use tui::style::Color;

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

        self.terminal.draw(|f| {

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                //.margin(1)
                .constraints(
                    [
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
                    //Constraint::Percentage(10)
                    ].as_ref()
                ).split(f.size());

            let block = Block::default()
                .title("Block")
                .borders(Borders::ALL);

//            let canvas = Canvas::default()
//                .block(block)
//                .paint(|ctx| {
//                    ctx.print(0.0, 0.0, "hello", Color::White);
//                });
                //.x_bounds([-1.0,1.0]).y_bounds([-1.0,1.0]);

            let canvas = Canvas::default()
                .block(block)
                .paint(|ctx| {
                    ctx.print(0.0, 0.0, "0", Color::White);
                    ctx.print(1.0, 1.0, "1", Color::White);
                    ctx.print(1.0, 3.0, "2", Color::White);
                })
                .x_bounds([0.0,5.0])
                .y_bounds([0.0,5.0]);


            //f.render_widget(block, chunks[0]);
            f.render_widget(canvas, chunks[0]);

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
