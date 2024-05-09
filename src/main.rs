
// today we are going to mess with ANSI colors
// I didn't know ANSI colors were a thing on the terminal and that's super cool

use ansi_term::{Color, Style};

fn print_red(msg: &str)
{
    print!("{}", Color::Red.paint(msg));
}

fn print_blue(msg: &str)
{
    print!("{}", Color::Blue.paint(msg))
}

fn print_green(msg: &str)
{
    print!("{}", Color::Green.paint(msg));
}

fn print_bold(msg: &str)
{
    print!("{}", Style::new().bold().paint(msg))
}

fn print_bold_and_red(msg: &str)
{
    print!("{}", Color::Red.bold().paint(msg));
}

fn main()
{
    println!("Hello World!\n");
    println!("in red:");
    print_red("Hello World");
    println!("\n");
    println!("in blue:");
    print_blue("Hello World");
    println!("\n");
    println!("in green:");
    print_green("Hello World");
    println!("\n");
    println!("in bold:");
    print_bold("Hello World");
    println!("\n");
    println!("in bold and green:");
    print_bold_and_red("Hello World");
    println!("\n");
}