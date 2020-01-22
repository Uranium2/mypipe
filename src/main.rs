extern crate clap;
use clap::{Arg, App};
use std::io::{self, Write};


fn main() {
    let matches = App::new("mypipe")
    .version("0.1")
    .author("Antoine TAVERNIE (Uranium2) <atavernier1@myges.fr>")
    .about("This program will run your commands thanks to pipes")
    .arg(Arg::with_name("input")
        .short("i")
        .long("input")
        .help("Input")
        .required(true)
        .takes_value(true)
    )
    .arg(Arg::with_name("output")
        .short("o")
        .long("output")
        .help("Output")
        .required(true)
        .takes_value(true)
    )                      
    .get_matches();
    let input_arg = matches.value_of("input").unwrap();
    let output_arg = matches.value_of("output").unwrap();

    let input_cmd = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
                .args(&["/C", input_arg])
                .output()
                .expect("failed to execute process")
    } else {
        std::process::Command::new("sh")
                .arg("-c")
                .arg(input_arg)
                .output()
                .expect("failed to execute process")
    };

    let output_cmd = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
                .args(&["/C", &String::from_utf8(input_cmd.stdout).unwrap()])
                .output()
                .expect("failed to execute process")
    } else {
        std::process::Command::new("sh")
                .arg("-c")
                .arg(&String::from_utf8(input_cmd.stdout).unwrap())
                .output()
                .expect("failed to execute process")
    };
    

}