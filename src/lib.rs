use rand::Rng;
use std::io::{self, Read, Result, Write};
use std::time::SystemTime;

struct State {
    answer: u8,
    problem: u8,
    now: SystemTime,
    presses: u32,
}

impl State {
    fn new() -> State {
        State {
            answer: 0,
            problem: rand::thread_rng().gen(),
            now: SystemTime::now(),
            presses: 0,
        }
    }

    fn print(&self) {
        escape_sequence(&"2J");
        escape_sequence(&"H");

        println!("--------------------");
        println!("Target number:   {:3}", self.problem);
        println!("{:08b}       = {:3}", self.answer, self.answer);
    }

    fn print_finish(&self) {
        let min_presses = self.problem.count_ones();

        println!(
            "You win! Finished in {}ms with {} press{}, min: {} press{}",
            self.now.elapsed().unwrap().as_millis(),
            self.presses,
            (if self.presses == 1 { "" } else { "es" }),
            min_presses,
            (if min_presses == 1 { "" } else { "es" }),
        );
    }

    fn guess(&mut self, n: u8) {
        self.presses += 1;
        self.answer ^= n;
    }

    fn is_solved(&self) -> bool {
        self.answer == self.problem
    }
}

pub fn run() -> Result<()> {
    setup_terminal()?;

    let mut state = State::new();

    loop {
        state.print();

        if state.is_solved() {
            state.print_finish();

            println!("Hit any key to continue...");
            get_char()?;

            state = State::new();
        } else {
            state.guess(get_guess()?);
        }
    }
}

fn get_guess() -> Result<u8> {
    Ok(match get_char()? {
        '1' => 0b1000_0000,
        '2' => 0b0100_0000,
        '3' => 0b0010_0000,
        '4' => 0b0001_0000,
        '5' => 0b0000_1000,
        '6' => 0b0000_0100,
        '7' => 0b0000_0010,
        '8' => 0b0000_0001,
        _ => 0,
    })
}

fn setup_terminal() -> Result<()> {
    const STDIN_FILENO: i32 = 0;
    let mut termios = termios::Termios::from_fd(STDIN_FILENO)?;
    termios.c_lflag &= !(termios::ICANON | termios::ECHO);
    termios::tcsetattr(0, termios::TCSANOW, &termios)?;
    Ok(())
}

fn get_char() -> Result<char> {
    let mut buffer = [0; 1];
    io::stdout().lock().flush()?;
    io::stdin().read_exact(&mut buffer)?;
    Ok(buffer[0] as char)
}

fn escape_sequence(value: &str) {
    print!("{}[{}", 27 as char, value);
}
