use std::io::{self, BufRead};

enum State {
    Normal,
    Escape,
    Csi,
}

fn print_conv(graph_end: &mut bool, v: &mut Vec::<char>, ch: char) {
    if *graph_end == false {
        *graph_end = ch.is_digit(16)
    }
    if *graph_end == true {
        v.push(ch);
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
    v.push(conv);
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
        let mut v = Vec::<char>::with_capacity(2048);
        for c in line_data.chars() {
            if graph_end {
                v.push(c);
                continue;
            }
            match &state {
                State::Normal => {
                    if c == 0x1B as char { // ESC
                        state = State::Escape;
                        v.push(c);
                    } else {
                        print_conv(&mut graph_end, &mut v, c);
                    }
                },
                State::Escape => {
                    v.push(c);
                    if c == 0x5B as char { // [
                        state = State::Csi;
                    } else {
                        state = State::Normal;
                    }
                },
                State::Csi => {
                    v.push(c);
                    if c >= 0x40 as char && c < 0x80 as char {
                        state = State::Normal;
                    }
                },
            }
        }
        let s: String = v.into_iter().collect();
        println!("{}", s);
    }
}

