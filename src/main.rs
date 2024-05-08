
// today I think we're going to make random values
// https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html

use rand::Rng; // for randomness
use text_io::read; // for reading user input
use std::process::Command; // for sending console commands

fn getu8() -> u8
{
    // create an initialized generator based on thread
    let mut rng = rand::thread_rng();

    // generate result based on generator
    let result: u8 = rng.gen();

    // return result
    result
}

fn getu16() -> u16
{
    // create an initialized generator based on thread
    let mut rng = rand::thread_rng();

    // generate result based on generator
    let result: u16 = rng.gen::<u16>();

    // return result
    result
}

fn getu32() -> u32
{
    // create an intialized generator based on thread
    let mut rng = rand::thread_rng();

    // generate result based on generator
    let result: u32 = rng.gen::<u32>();

    // return result
    result
}

fn geti32() -> i32
{
    // create an initialized generator based on thread
    let mut rng = rand::thread_rng();

    // generate result based on generator
    let result: i32 = rng.gen::<i32>();

    // return result
    result
}

fn getfloat() -> f64
{
    // create an initialized generator based on thread
    let mut rng = rand::thread_rng();

    // generate result based on generator
    let result: f64 = rng.gen::<f64>();

    // return result
    result
}

fn clear()
{
    // if on windows
    if cfg!(windows)
    {
        // send clear command to cmd
        Command::new("cmd").args(["/c", "cls"]).spawn().expect("cls failed to start").wait().expect("failed to wait for cls");
    }
    else
    {
        // if on linux, send clear command
        Command::new("clear").spawn().expect("clear command failed to start").wait().expect("failed to wait");
    }
}
fn main() 
{
    // while(true)
    loop
    {
        // print random
        println!("random u8: {}", getu8());
        println!("random u16: {}", getu16());
        println!("random u32: {}", getu32());
        println!("random i32: {}", geti32());
        println!("random float: {}", getfloat());

        // read user input
        let _input: String = read!("{}\n");

        // clear terminal
        clear();
    }
}