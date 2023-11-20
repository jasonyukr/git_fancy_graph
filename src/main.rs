use std::io::{self, BufRead};

const STRIP_COLOR: bool = false;

enum State {
    Normal,
    Escape,
    Csi,
}

fn print_conv(graph_end: &mut bool, ch: char) {
    if *graph_end == false {
        *graph_end = ch.is_digit(16)
    }
    if *graph_end == true {
        print!("{}", ch);
        return;
    }

    // make target characters as bold
    match ch {
        '|'  => { 
            print!("\x1b[1m{}\x1b[0m", '|');
        },
        '/'  => { 
            print!("\x1b[1m{}\x1b[0m", '/');
        },
        '\\' => {
            print!("\x1b[1m{}\x1b[0m", '\\');
        },
        '_' => {
            print!("\x1b[1m{}\x1b[0m", '_');
        },
        '*'  => {
            print!("\x1b[1m{}\x1b[0m", '\u{2b2e}'); // ⬮
        },
        _    => {
            print!("{}", ch);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line_data;
        match line {
            Ok(data) => line_data = data,
            Err(_) => continue
        }
        let mut state = State::Normal;
        let mut graph_end = false;
        for c in line_data.chars() {
            match &state {
                State::Normal => {
                    if c == 0x1B as char { // ESC
                        state = State::Escape;
                        if !STRIP_COLOR {
                            print!("{}", c);
                        }
                    } else {
                        print_conv(&mut graph_end, c);
                    }
                },
                State::Escape => {
                    if !STRIP_COLOR {
                        print!("{}", c);
                    }
                    if c == 0x5B as char { // [
                        state = State::Csi;
                    } else {
                        state = State::Normal;
                    }
                },
                State::Csi => {
                    if !STRIP_COLOR {
                        print!("{}", c);
                    }
                    if c >= 0x40 as char && c < 0x80 as char {
                        state = State::Normal;
                    }
                },
            }
        }
        println!();
    }
}

