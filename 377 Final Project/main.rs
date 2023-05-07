mod inverter;

// use std::collections::{BTreeMap, HashSet};
use std::env;
// use std::fs::File;
use std::io::{self};
use std::process;



fn main() -> io::Result<()> {
    // Read command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Wrong number of inputs, need two.");
        process::exit(1);
    }


    // Call the function and print the result
    println!("{}", inverter::build_inverted_index(&args[1])?);

    Ok(())
}