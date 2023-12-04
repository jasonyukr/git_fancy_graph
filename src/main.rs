use std::io::{self, BufRead};

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

    let conv;
    match ch {
        '|'  => conv = '\u{007c}', // | 
        '/'  => conv = '\u{2571}', // ╱
        '\\' => conv = '\u{2572}', // ╲
        '*'  => conv = '\u{2b2e}', // ⬮
        _    => conv = ch
    }
    print!("{}", conv);
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
            if graph_end {
                print!("{}", c);
                continue;
            }
            match &state {
                State::Normal => {
                    if c == 0x1B as char { // ESC
                        state = State::Escape;
                        print!("{}", c);
                    } else {
                        print_conv(&mut graph_end, c);
                    }
                },
                State::Escape => {
                    print!("{}", c);
                    if c == 0x5B as char { // [
                        state = State::Csi;
                    } else {
                        state = State::Normal;
                    }
                },
                State::Csi => {
                    print!("{}", c);
                    if c >= 0x40 as char && c < 0x80 as char {
                        state = State::Normal;
                    }
                },
            }
        }
        println!();
    }
}

