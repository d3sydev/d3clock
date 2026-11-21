use std::{
    io::{self, Write},
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

const DIGITS: [[&str; 5]; 11] = [
    ["███", "█ █", "█ █", "█ █", "███"],
    ["  █", "  █", "  █", "  █", "  █"],
    ["███", "  █", "███", "█  ", "███"],
    ["███", "  █", "███", "  █", "███"],
    ["█ █", "█ █", "███", "  █", "  █"],
    ["███", "█  ", "███", "  █", "███"],
    ["███", "█  ", "███", "█ █", "███"],
    ["███", "  █", "  █", "  █", "  █"],
    ["███", "█ █", "███", "█ █", "███"],
    ["███", "█ █", "███", "  █", "███"],
    ["   ", " █ ", "   ", " █ ", "   "],
];

fn now() -> (String, String) {
    let out = Command::new("date")
        .arg("+%H:%M:%S|%A, %d %b %Y")
        .output()
        .expect("date failed");
    let raw = String::from_utf8_lossy(&out.stdout);
    let mut parts = raw.trim().split('|');
    (
        parts.next().unwrap_or("00:00:00").to_string(),
        parts.next().unwrap_or("").to_string(),
    )
}

fn term_size() -> (usize, usize) {
    let Ok(out) = Command::new("stty")
        .arg("size")
        .stdin(Stdio::inherit())
        .output()
    else {
        return (24, 80);
    };

    let s = String::from_utf8_lossy(&out.stdout);
    let mut nums = s.split_whitespace().filter_map(|n| n.parse::<usize>().ok());

    match (nums.next(), nums.next()) {
        (Some(r), Some(c)) => (r, c),
        _ => (24, 80),
    }
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("ctrlc handler error");

    let mut out = io::stdout();
    print!("\x1b[?1049h\x1b[?25l");
    let _ = out.flush();

    while running.load(Ordering::SeqCst) {
        let (time_str, date_str) = now();
        let (rows, cols) = term_size();

        let mut buf = [String::new(), String::new(), String::new(), String::new(), String::new()];
        for ch in time_str.chars() {
            let idx = match ch {
                '0'..='9' => ch as usize - '0' as usize,
                ':' => 10,
                _ => continue,
            };
            for row in 0..5 {
                buf[row].push_str(DIGITS[idx][row]);
                buf[row].push_str("  ");
            }
        }

        let pad_left = cols.saturating_sub(38) / 2;
        let pad_top = rows.saturating_sub(7) / 2;
        let date_pad = cols.saturating_sub(date_str.len()) / 2;

        print!("\x1b[2J\x1b[H");
        for _ in 0..pad_top {
            println!();
        }

        for line in &buf {
            println!("{}\x1b[1;35m{}\x1b[0m", " ".repeat(pad_left), line.trim_end());
        }

        println!("\n{}\x1b[2;37m{}\x1b[0m", " ".repeat(date_pad), date_str);
        let _ = out.flush();

        thread::sleep(Duration::from_millis(200));
    }

    print!("\x1b[?25h\x1b[?1049l");
    let _ = out.flush();
}

