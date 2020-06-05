use chrono::prelude::*;
use clap::{App, Arg};
// use std::process::exit;

fn main() {
    println!("Hello, world!");

    // TODO: читаем аргументы командной строки
    let matches = App::new("arj")
        .version("0.0.1")
        .author("nikonor <nikonor@nikonor.ru>")
        .about("Personal document archive")
        .arg(
            Arg::with_name("init")
                .short("i")
                .long("init")
                .takes_value(true)
                .help("<archive directory>"),
        )
        // .arg(
        //     Arg::with_name("num")
        //         .short("n")
        //         .long("number")
        //         .takes_value(true)
        //         .help("Five less than your favorite number"),
        // )
        .get_matches();

    let dir = matches.value_of("init"); //.unwrap_or("./");
    match dir {
        Some(dir) => println!("The init passed is: {}", dir),
        None => println!("{}", matches.usage()),
    }

    let year = Local::now().date().year();
    println!("year={}", year);
}
