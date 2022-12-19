mod app;
mod event;
mod game;
mod utils;

///These lines import four modules (Rust's way of organizing code into reusable units) with the names 
/// `app`, `event`, `game`, and `utils`.
/// This Rust program is a command-line implementation of the popular game 2048. The program imports 
/// various modules and external crates that provide functionality for user input, terminal output, and game logic.
///  The main function sets up a terminal and an event loop, and on each iteration of the loop it calls the draw method
///  on the terminal to render the game's user interface. The draw method takes a closure as an argument, which is used 
/// to draw various elements of the user interface, such as a canvas for the game board and text for the score and current
///  game command. The game logic is implemented in the App struct, which is created and updated in the event loop. 
/// The loop also handles user input events, such as key presses, and uses them to update the game state and trigger 
/// actions such as moving tiles or quitting the game.

use std::{error::Error, io, time::Duration};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color},
    widgets::{
        canvas::{Canvas, Line, Points},
        Block, Borders,
    },
    Terminal,
};

use app::App;
use event::{Config, Event, Events};
use game::Command;

// These lines use various items (such as types, functions, and constants) from the
// Rust standard library and external crates (collections of Rust code that are available for use in other projects).

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let config = Config {
        tick_rate: Duration::from_millis(250),
        ..Default::default()
    };
    let events = Events::with_config(config);

    let mut app = App::new();

    loop {
        // This loop will run indefinitely, and on each iteration it will call the draw method on a Terminal object 
        // to render the game's user interface. The draw method takes a closure (a function that can be passed as an
        //  argument to another function) as an argument. This closure has a single argument, f, which represents the
        //  frame that the user interface will be drawn on. The let statement creates a variable called chunks which 
        // is an array of two Rect objects that represent the left and right halves of the terminal window. 
        // This array is created using the Layout type, which is part of the tui crate.
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());
            // params
            let board_size = app.get_size();
            let panel_size = board_size + (board_size / 3.0);
            let half_box_size = app.box_size / 2.0;
            let font_width = 2.0;
            // Game board
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("2048-@wander"))
                .paint(|ctx| {
                    let grid = app.get_grid();
                    for (row, list) in grid.iter().enumerate() {
                        for (col, _) in list.iter().enumerate() {
                            // 盒子参数
                            let score = grid[row][col];
                            let s = if score == 0 {
                                String::from("").into_boxed_str()
                            } else {
                                pad_str(score.to_owned().to_string(), 6).into_boxed_str()
                            };
                            let x_box = (col as f64) * app.box_size;
                            let y_box = (row as f64) * app.box_size;
                            ctx.print(
                                ((col + 1) as f64) * app.box_size - half_box_size - font_width,
                                ((4 - row) as f64) * app.box_size
                                    - half_box_size
                                    - font_width * 2.0,
                                Box::leak(s),
                                score_to_color(score),
                            );
                            ctx.draw(&Line {
                                x1: x_box,
                                y1: y_box,
                                x2: x_box + app.box_size,
                                y2: y_box,
                                color: Color::Green,
                            });
                            ctx.draw(&Line {
                                x1: x_box,
                                y1: y_box,
                                x2: x_box,
                                y2: y_box + app.box_size,
                                color: Color::Green,
                            });
                            ctx.draw(&Line {
                                x1: x_box + app.box_size,
                                y1: y_box,
                                x2: x_box + app.box_size,
                                y2: y_box + app.box_size,
                                color: Color::Green,
                            });
                            ctx.draw(&Line {
                                x1: x_box,
                                y1: y_box + app.box_size,
                                x2: x_box + app.box_size,
                                y2: y_box + app.box_size,
                                color: Color::Green,
                            });
                        }
                    }

                    if !app.is_alive() {

                        ctx.draw(&Points {
                            coords: &app.get_game_over_modal(),
                            color: Color::Green
                        });

                        ctx.print(
                            app.box_size * 1.5,
                            app.box_size * 2.0,
                            " GAME OVER! ",
                            Color::Blue,
                        );

                        ctx.print(
                            app.box_size * 1.3,
                            app.box_size * 1.8,
                            " Restart[R] Quit[Q] ",
                            Color::Blue,
                        );
                    }
                })
                .x_bounds([0.0, board_size])
                .y_bounds([0.0, board_size]);
            f.render_widget(canvas, chunks[0]);
            // Informantions
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Panel"))
                .paint(|ctx| {
                    ctx.print(board_size, board_size, "> Relax <", Color::Blue);

                    let score = app.get_score().to_owned().to_string().into_boxed_str();
                    ctx.print(board_size, board_size - 30.0, "Score:", Color::Green);
                    ctx.print(
                        board_size,
                        board_size - 40.0,
                        Box::leak(score),
                        Color::Green,
                    );

                    ctx.print(board_size, 0.0, "Quit[Q]", Color::Blue);
                })
                .x_bounds([board_size, panel_size])
                .y_bounds([0.0, board_size]);
            f.render_widget(canvas, chunks[1]);
        })?;

        // Events
        // handling user input events for the game. It uses a match statement to 
        // determine what action to take based on the type of event that occurs. 
        // If the event is an Input event, then this block of code will be executed.
        //  It uses another match statement to handle different types of input events.
        //  If the input event is a character event with the value 'q', the loop will 
        // be broken and the program will exit. If the input event is a character event
        //  with the value 'r', the restart method on the app object will be called.

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Char('r') => {
                    app.restart();
                }
                // left up right down
                // This block of code handles various arrow key events and vim-style key events. 
                // If an arrow key event or a vim-style key event is detected, it will add a 
                // corresponding Command value to the app object using the add_command method. 
                // The Command values represent the different actions that can be taken in the game,
                // such as moving a tile up, down, left, or right.

                Key::Down => {
                    app.add_command(Command::Down);
                }
                Key::Up => {
                    app.add_command(Command::Up);
                }
                Key::Right => {
                    app.add_command(Command::Right);
                }
                Key::Left => {
                    app.add_command(Command::Left);
                }
                // h k l j   vim keys support
                Key::Char('h') => {
                    app.add_command(Command::Left);
                }
                Key::Char('k') => {
                    app.add_command(Command::Up);
                }
                Key::Char('l') => {
                    app.add_command(Command::Right);
                }
                Key::Char('j') => {
                    app.add_command(Command::Down);
                }
                _ => {
                    app.add_command(Command::Nil);
                }
            },
            Event::Tick => {
                app.next()
            }
        }
    }

    Ok(())
}

// make different strings as same length
// The function first makes a copy of s and stores it in a variable called s. 
// It then enters a loop. Within the loop, the function checks whether the length of s is less than length. 
// If it is, it appends a space character to the end of s. If the length of s is equal to or greater than length, 
// the loop breaks and the function returns s.
// This function can be used to pad a string with spaces so that it reaches a minimum length. 
// For example, if the input string is "hello" and the desired length is 10, the function would return "hello " 
// (with three spaces added to the end). If the input string is already at least as long as the desired length, 
// the function will return the string unmodified.
fn pad_str(s: String, length: usize) -> String {
    let mut s = s.clone();
    loop {
        if s.len() < length {
            s.push_str(" ");
        } else {
            break;
        }
    }

    s
}

// render different color for different score
// uses a series of if statements to determine which Color to return based on the value of score. 
// If score is less than 64, the function returns Color::Green. If score is greater than or equal to 64 but less than 256,
//  the function returns Color::Magenta. If score is greater than or equal to 256 but less than 1024, 
// the function returns Color::Cyan. If score is greater than or equal to 1024 but less than 4096, 
// the function returns Color::LightRed. If none of these conditions are met, the function returns Color::Red.
fn score_to_color(score: i32) -> Color {
    if score < 64 {
        Color::Green
    } else if score < 256 {
        Color::Magenta
    } else if score < 1024 {
        Color::Cyan
    } else if score < 4096 {
        Color::LightRed
    } else {
        Color::Red
    }
}
