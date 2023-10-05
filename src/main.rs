use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use rand_word;
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
};
use ratatui::{
    style::{Color, Style},
    text::Span,
};
use std::error::Error;
use std::io::{self, Stdout};
use std::time::{Duration, SystemTime};

mod utils;
use utils::{get_accuracy, get_wpm};
// #[warn(unused_variables)]

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    // let is_details_focus = ;
    // let is_check_focus = ;
    let words = 20;
    let mut typing: String = rand_word::new(words);
    let mut input_writing = String::new();
    // let mut count = 0;
    // let mut check = 0;
    let mut start: Option<SystemTime> = None;
    let mut end: Option<u64> = None;
    Ok(loop {
        terminal.draw(|frame| {
            //    time vala variable and wpm wala yaha pe declare karna

            if typing.len() == input_writing.len() {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(10),
                            Constraint::Min(1),
                            Constraint::Percentage(90),
                        ]
                        .as_ref(),
                    )
                    .split(frame.size());
                if end.is_none() {
                    end = match start.unwrap().elapsed() {
                        Ok(elapsed) => Some(elapsed.as_secs()),
                        Err(_) => Some(0),
                    };
                }

                let better_luck =
                    Paragraph::new("Try faster next time").alignment(Alignment::Center);
                // let block = Block::default().title("OP").borders(Borders::ALL);
                frame.render_widget(better_luck, chunks[0]);

                let block = Paragraph::new(vec![
                    Line::from(format!(
                        "Accuracy {:.2}",
                        get_accuracy(&typing, &input_writing)
                    )),
                    Line::from(format!(
                        "WPM {}",
                        (words as f32 / end.unwrap() as f32) * 60.0
                    )),
                    Line::from(format!(" ")),
                    Line::from(format!(" ")),
                    Line::from("Keybindings : "),
                    Line::from("1.  TAB -> Reset "),
                    Line::from("2.  Enter -> Restart"),
                    Line::from("3.  Backspace -> Remove the last element "),
                ])
                .block(Block::default().title("Result").borders(Borders::ALL));
                frame.render_widget(block, chunks[2]);
            } else {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(10),
                            Constraint::Min(2),
                            Constraint::Percentage(90),
                        ]
                        .as_ref(),
                    )
                    .split(frame.size());
                let mut goal = Vec::new();
                let typing_vec = typing.chars().collect::<Vec<char>>();

                for (i, ch) in input_writing.char_indices() {
                    let color = if ch == typing_vec[i] {
                        Color::Green
                    } else {
                        Color::Red
                    };
                    //  let block =Paragraph::new("Welcome to cockroachtype!!!!!!"). Block::default().title("Hello").borders(Borders::ALL).alignment(Alignment::Center);
                    // frame.render_widget(block, chunks[0]);
                    // let block = Block::default().title("Hello").borders(Borders::ALL);
                    // frame.render_widget(block, chunks[0]);

                    goal.push(Span::styled(
                        if typing_vec[i] != ' ' {
                            typing_vec[i].to_string()
                        } else {
                            ch.to_string()
                        },
                        Style::default().fg(color),
                    ))
                }

                for i in (input_writing.len())..(typing.len()) {
                    goal.push(Span::styled(
                        typing.chars().nth(i).unwrap().to_string(),
                        Style::default().fg(Color::DarkGray),
                    ))
                }

                // let block =
                // Paragraph::new("WELCOME TO COCKROACHTYPE!!!!").alignment(Alignment::Center);

                let block = Paragraph::new(vec![

                Line::from(format!(
                "Accuracy : {:.2} ", get_accuracy(&typing , & input_writing)
            )),

                    Line::from( "  "),

                        Line::from(format!("WPM: {:.2}" ,  if start.is_some() {
                        get_wpm(start.unwrap(), &input_writing)
                    } else {
                        0f64
                    }
                            )



                        
)
                    // format!(
                    // "Accuracy: {:.2} | WPM: {:.2}",
                    // get_accuracy(&typing, &input_writing),
                    // if start.is_some() {
                    //     get_wpm(start.unwrap(), &input_writing)
                    // } else {
                    //     0f64
                    // }
                ]).alignment(Alignment::Center);
                frame.render_widget(block, chunks[0]);

                let block = Paragraph::new(Line::from(goal))
                    .wrap(Wrap { trim: true })
                    .block(Block::default().title("Type here").borders(Borders::ALL));
                frame.render_widget(block, chunks[2]);
            }
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let event::Event::Key(key) = event::read()? {
                if event::KeyCode::Char('1') == key.code {
                    break;
                }
                if let event::KeyCode::Char(ch) = key.code {
                    if input_writing.len() < typing.len() {
                        input_writing.push(ch);
                        // if ch == typing.chars().nth(check).unwrap() {
                        //     count += 1;
                        // }
                        // if check < typing.len() - 1 {
                        //     check += 1;
                        // }
                        if input_writing.len() == 1 {
                            start = Some(SystemTime::now());
                        }
                    }
                }

                if key.code == event::KeyCode::Backspace {
                    // if input_writing.chars().nth(check) == typing.chars().nth(check) {
                    //     count -= 1;
                    // }

                    if !input_writing.is_empty() {
                        input_writing.pop();
                    }
                    // check -= 1;
                }
                if key.code == event::KeyCode::Esc {
                    break;
                }
                if key.code == event::KeyCode::Tab {
                    input_writing = "".to_string();
                    typing = rand_word::new(words);
                    while typing.contains("º") {
                        typing = rand_word::new(words);
                    }
                    // count = 0;
                    // check = 0;
                }
                if key.code == event::KeyCode::Enter {
                    input_writing = "".to_string();
                    // count = 0;
                    // check = 0;
                }
            }
        }
    })
}
