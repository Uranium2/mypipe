extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("mypipe")
        .version("0.1")
        .author("Antoine TAVERNIE (Uranium2) <tavernier.antoine@hotmail.fr>")
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

    let input_out = if cfg!(target_os = "windows") {
        std::process::Command::new(input_arg)
        .output()
        .expect("failed to execute process")
    } else {
        std::process::Command::new(input_arg)
        .output()
        .expect("failed to execute process")
    };
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new(output_arg)
        .arg(String::from_utf8(input_out.stdout).unwrap())
        .output()
        .expect("failed to execute process")
    } else {
        std::process::Command::new(output_arg)
        .arg(String::from_utf8(input_out.stdout).unwrap())
        .output()
        .expect("failed to execute process")
    };

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        print!("{}", result);
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        print!("The progam failed, the error was: \n{}", error);
    }
}