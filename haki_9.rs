use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let period = &s[s.len() - 2..];
    let hour: i32 = s[0..2].parse().expect("Invalid hour format");

    let new_hour = match period {
        "AM" if hour == 12 => 0,
        "PM" if hour != 12 => hour + 12,
        _ => hour,
    };

    format!("{:02}:{:02}:{:02}", new_hour, &s[3..5], &s[6..8])
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}