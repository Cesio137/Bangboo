use colored::Colorize;

pub fn success(message: &str) {
    println!("{} {message}", "✔".bright_green());
}

pub fn error(message: &str) {
    println!("{} {message}", "✘".bright_red());
}

pub fn warn(message: &str) {
    println!("{} {message}", "⚠".bright_yellow());
}

pub fn log(message: &str) {
    println!("{message}");
}
