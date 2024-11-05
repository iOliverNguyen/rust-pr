use anyhow::Result;

pub fn print_title(msg: &str) {
    println!("{}", format_title(msg));
}

pub fn format_title(msg: &str) -> String {
    const MAX: usize = 50;
    const MAX_MSG: usize = 50 - 10;

    let mut s = "".to_string();
    if msg.len() == 0 {
        return "_".repeat(MAX);
    }
    if msg.len() >= MAX_MSG {
        return format!("---- {} ----", msg);
    }
    let pad = (MAX - msg.len()) / 2;
    for _ in 0..(pad - 1) {
        s.push('-')
    }
    s.push(' ');
    s.push_str(msg);
    s.push(' ');
    for i in 0..(MAX - pad - msg.len() - 1) {
        s.push('-')
    }
    s
}
