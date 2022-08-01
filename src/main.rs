use std::{io::{stdout}, time::Duration, collections::VecDeque};

use crossterm::{
    execute, 
    style::{SetForegroundColor, Print, ResetColor, Color}, 
    Result, terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode}, cursor, event::{poll, read, Event, KeyCode}};
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
struct Pos(u16, u16);

enum Direction {
    Left,
    Right,
    Up,
    Down
}

const BOARD_SIZE: u16 = 20;
const INIT_POS: Pos = Pos(BOARD_SIZE/2, BOARD_SIZE/2);

fn print_board() -> Result<()> {
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            execute!(
                stdout(),
                cursor::MoveTo(x * 2, y),
                Print(".")
            )?;
        }
    }
    Ok(())
}

fn main() -> Result<()>{
    enable_raw_mode()?;

    execute!(
        stdout(),
        Clear(ClearType::All),
        cursor::Hide,
        
    )?;

    
    
    update()?;
    
    disable_raw_mode()?;

    execute!(
        stdout(),
        cursor::Show
    )?;
    
    println!("you lost lmao L");

    Ok(())
}

fn update() -> Result<()>{
    let mut snake_pos: Pos           = INIT_POS;
    let mut snake_dir       = Direction::Right;
    let mut  prev_pos: VecDeque<Pos> = VecDeque::new();
    let mut snake_length        = 2;
    let mut food_pos             = Pos(3,3);
    'main: loop {
        print_board()?;
        for (idx,i) in prev_pos.clone().iter().enumerate() {
            if idx > snake_length {
                continue;
            }
            if *i == snake_pos {
                break 'main;
            }
            execute!(stdout(),
                cursor::MoveTo(i.0 * 2, i.1),
                SetForegroundColor(Color::DarkGreen),
                Print("O"),
                ResetColor
            )?;
        }

        execute!(stdout(),
            cursor::MoveTo(food_pos.0 * 2, food_pos.1),
            SetForegroundColor(Color::Red),
            Print("@"),
            ResetColor
        )?;

        execute!(
            stdout(),
            cursor::MoveTo(snake_pos.0 * 2, snake_pos.1),
            SetForegroundColor(Color::Green),
            Print("#"),
            ResetColor
        )?;

        if poll(Duration::from_millis(250))? {
            let event = read()?;
            // println!("Event::{:?}\r", event);

            if event == Event::Key(KeyCode::Char('q').into()) {
                break;
            }
            if event == Event::Key(KeyCode::Up.into()) {
                snake_dir = Direction::Up;
            }
            if event == Event::Key(KeyCode::Down.into()) {
                snake_dir = Direction::Down;
            }

            if event == Event::Key(KeyCode::Left.into()) {
                snake_dir = Direction::Left;
            }
            if event == Event::Key(KeyCode::Right.into()) {
                snake_dir = Direction::Right;
            }
        }

        prev_pos.push_front(snake_pos);

        match snake_dir {
            Direction::Left     => {
                if snake_pos.0 == 0 {
                    break;
                }
                else {
                    snake_pos.0 -= 1;
                }
            },
            Direction::Right    => {
                if snake_pos.0 == BOARD_SIZE - 1 {
                    break;
                }
                else {
                    snake_pos.0 += 1;
                }
            },
            Direction::Up       => {
                if snake_pos.1 == 0 {
                    break;
                }
                else {
                    snake_pos.1 -= 1;
                }
            },
            Direction::Down     => {
                if snake_pos.1 == BOARD_SIZE - 1 {
                    break;
                }
                else {
                    snake_pos.1 += 1;
                }
            },
        }

        if snake_pos == food_pos {
            snake_length += 1;
            let mut thread_rng = rand::thread_rng();
            let x = thread_rng.gen_range(0..BOARD_SIZE);
            let y = thread_rng.gen_range(0..BOARD_SIZE);
            food_pos = Pos(x, y);
        }

        // std::thread::sleep(Duration::from_millis(250));
    };

    Ok(())
}
