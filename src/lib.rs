#![feature(duration_as_u128)]

use rand::Rng;
use std::{
    io::{self, Read, Result, Write},
    time::SystemTime,
};

struct State {
    answer: u8,
    problem: u8,
    now: SystemTime,
}

impl State {
    fn new() -> State {
        State {
            answer: 0,
            problem: rand::thread_rng().gen(),
            now: SystemTime::now(),
        }
    }
}

pub fn run() -> Result<()> {
    setup_terminal()?;

    let mut state = State::new();

    loop {
        print(state.answer, state.problem);

        if state.answer == state.problem {
            println!("You win!");
            println!("Hit any key to continue...");
            println!("{}ms", state.now.elapsed().unwrap().as_millis());

            get_char()?;

            state = State::new();
        } else {
            state.answer ^= match get_char() {
                Ok('1') => 0b1000_0000,
                Ok('2') => 0b0100_0000,
                Ok('3') => 0b0010_0000,
                Ok('4') => 0b0001_0000,
                Ok('5') => 0b0000_1000,
                Ok('6') => 0b0000_0100,
                Ok('7') => 0b0000_0010,
                Ok('8') => 0b0000_0001,
                _ => 0,
            };
        }
    }
}

fn print(answer: u8, problem: u8) {
    escape_sequence(&"2J");
    escape_sequence(&"H");

    println!("--------------------");
    println!("Answer to match: {:3}", problem);
    println!("{:08b}       = {:3}", answer, answer);
}

fn setup_terminal() -> Result<()> {
    let mut termios = termios::Termios::from_fd(libc::STDIN_FILENO)?;
    termios.c_lflag &= !(termios::ICANON | termios::ECHO);
    termios::tcsetattr(0, termios::TCSANOW, &termios)?;
    Ok(())
}

fn get_char() -> Result<char> {
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0; 1];

    stdout.lock().flush()?;
    reader.read_exact(&mut buffer)?;
    Ok(buffer[0] as char)
}

fn escape_sequence(value: &str) {
    print!("{}[{}", 27 as char, value);
}
