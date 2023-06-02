use std::io::{self, BufRead};

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
        '*'  => conv = '\u{25cf}', // ●
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
        let mut graph_end = false;
        for ch in line_data.chars() {
            print_conv(&mut graph_end, ch);
        }
        println!();
    }
}

