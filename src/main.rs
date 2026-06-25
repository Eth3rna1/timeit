/*!
    Time a command line process
*/
use std::env;
use std::io::Result;
use std::process::Command;
use std::process::exit;
use std::time::{Duration, Instant};

const HELP: &str = r#"Time a command line process

Usage: timeit <COMMANDS>"#;

const SECONDS: &'static str = "s";
const MINUTES: &'static str = "m";
const HOURS: &'static str = "h";

fn process(arguments: &[String]) -> Result<()> {
    let mut handle = Command::new("cmd").arg("/C").args(arguments).spawn()?;
    handle.wait()?;
    Ok(())
}

/// Returns (time, unit)
fn translate_time(time: Duration) -> (f64, &'static str) {
    let new_time: f64 = time.as_secs_f64();
    if new_time >= 60f64 {
        return (new_time / 60f64, MINUTES);
    } else if new_time >= 3600f64 {
        return (new_time / 3600f64, HOURS);
    }
    (new_time, SECONDS)
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() <= 1 {
        eprintln!("{}", HELP);
        exit(1)
    }
    let start = Instant::now();
    // I don't care about the end result
    let _ = process(&arguments[1..]);
    let end = Instant::now();
    let time = end - start;
    match time.as_secs() {
        0u64 => println!("\nExecution time of `{}`: {:?}", &arguments[1], time),
        _ => {
            let (time, unit) = translate_time(time);
            println!(
                "\nExecution time of `{}`: {:0.4}{}",
                &arguments[1], time, unit
            )
        }
    };
}
