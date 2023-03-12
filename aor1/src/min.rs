use crossterm::event::{DisableMouseCapture, EnableMouseCapture,Event,KeyCode,read};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;
use tui::backend::Backend;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders};
use tui::Terminal;
use tui_textarea::{Input, Key, TextArea};
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout,Alignment};
use tui::widgets::{Paragraph,Wrap};
use tui::text::{Text,Span,Spans};
use tui::style::{Style,Modifier,Color};

use std::fs;


fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let res = run_app(&mut terminal);


    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    //println!("Lines: {:?}", textarea.lines());
    Ok(())
}

fn run_app<B: Backend>(term: &mut Terminal<B>) -> io::Result<()> {

    /* ps 1 */
    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Crossterm Minimal Example"),
    );
    

    loop {
        let st = textarea.lines();
        let mut res:Vec<String> = Vec::new();

        if st.len() > 0 {
            res = st.iter()
            .map(|s| s.len().to_string())
            .collect::<Vec<String>>();
        } else  {
            res.push("0".to_string());
        }
        term.draw(|f| ui(f,&textarea,&res))?;

        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => {return Ok(());},
            Input {
                key: Key::Char('s'),
                ctrl: true,
                ..
            } => {
                std::fs::write("saved.txt", st.join("\n")).expect("failed to write to file");
            },
            input => {
                textarea.input(input);
            }
        }
    }


}


fn ui<B: Backend>(f: &mut Frame<B>,textarea:&TextArea,stack:&Vec<String>) {

    let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
        [
            Constraint::Percentage(80),
            Constraint::Percentage(20),
        ]
        .as_ref(),
    )
    .split(f.size());

f.render_widget(textarea.widget(), chunks[0]);
//let block = Block::default().title("Block 2").borders(Borders::ALL);
//f.render_widget(block, chunks[1]);
let mut spantext: Vec<Spans> = Vec::new();
for st in stack.iter(){
    spantext.push(Spans::from(Span::styled(st,Style::default().add_modifier(Modifier::ITALIC))));
}
/* 
let text = vec![
    Spans::from(vec![
        Span::raw("First"),
        Span::styled("line",Style::default().add_modifier(Modifier::ITALIC)),
        Span::raw("."),
    ]),
    Spans::from(Span::styled("Second line", Style::default().fg(Color::Red))),
];
*/
let para = Paragraph::new(spantext)
    .block(Block::default().title("Stack").borders(Borders::ALL))
    .style(Style::default().fg(Color::White).bg(Color::Black))
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });
f.render_widget(para, chunks[1]);
}