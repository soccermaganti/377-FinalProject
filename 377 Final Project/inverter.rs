
// 1) Rust Uses snake_case name

//2) In Rust, the File type implements the Drop trait, 
//which means that it will be automatically closed when it goes out of scope. 
//Therefore, you don't need to manually close the cur_file in your code.

//3)
//A BTreeMap is a map implementation in Rust that uses a balanced binary search tree 
//(specifically, a B-tree) to store its elements. The keys are stored in the tree in sorted 
//order, which allows for efficient lookup, insertion, and deletion operations while
//maintaining the sorted order of the keys.

//When you iterate over the keys of a BTreeMap, they will be returned in sorted order 
//according to their natural ordering, which is determined by the implementation of the Ord 
//trait for the key type. This means that you don't need to manually sort the keys of a 
//BTreeMap as they are already sorted by default.

//4) In Rust, unwrap is a method that can be called on a Result or Option type to 
//retrieve the inner value if it is Ok or Some respectively, and panic if it is Err or 
//None respectively.

//When you "unwrap" a line in the context of reading lines from a file using a 
//BufRead reader, it means that you are extracting the inner value from the Result type 
//returned by the lines() method, assuming it is Ok, and panicking if it is Err.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

//Function to print a BTreeSet of usize values to a String
fn print_set(s: &BTreeSet<usize>, out: &mut String) {
    for item in s {
        out.push_str(&format!(" {}", item));
    }
    out.push('\n');
}



fn index_inserter(word: &str, inverted_index: &mut BTreeMap<String, BTreeSet<usize>>,
    file_number: usize) {
    inverted_index
        .entry(word.to_string())
        .or_insert_with(BTreeSet::new)
        .insert(file_number);
}

// Function to build an inverted index from a list of files
fn build_inverted_index(filename: &str) -> io::Result<String> {
    // Open the file list and create a BufRead reader
    let file_list_reader = io::BufReader::new(File::open(filename)?);

    // Create a BTreeMap to store the inverted index
    let mut inverted_index: BTreeMap<String, BTreeSet<usize>> = BTreeMap::new();
    let mut file_number = 0;

    // Iterate over each file in the file list
    for file in file_list_reader.lines() {
        // Open the current file and create a BufRead reader
        let cur_file_reader = io::BufReader::new(File::open(&file?)?);

        // Iterate over each line in the current file
        for line in cur_file_reader.lines() {
            let line = line?; // Unwrap the line

            let mut word_start = 0;
            // Iterate over the characters in the line
            for (i, cur_char) in line.char_indices() {
                if !((cur_char >= 'a' && cur_char <= 'z') || cur_char >= 'A' && cur_char <= 'Z') {
                    // Extract the word between non-alphabetic characters
                    if i == word_start {
                        word_start += 1;
                    } else {
                        let word = &line[word_start..i];
                        word_start = i + 1;
                        // Insert the word into the inverted index
                        index_inserter(word, &mut inverted_index, file_number);
                    }
                }
            }

            // Insert the last word in the line, if any
            if word_start != line.len() {
                let word = &line[word_start..];
                index_inserter(word, &mut inverted_index, file_number);
            }
        }

        file_number += 1;
    }

     // Create a String to store the output
     let mut output = String::new();

     //Iterate over the keys (words) of the inverted index in sorted order
     for word in inverted_index.keys() {
         let file_numbers = inverted_index.get(word).unwrap(); // Safe to unwrap as word is guaranteed to be in inverted_index
         output.push_str(&format!("{}:", word));
         print_set(&file_numbers, &mut output);
     }
     Ok(output)
}

fn main() -> io::Result<()> {
    // Read command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Wrong number of inputs, need two.");
        process::exit(1);
    }

    // Call the build_inverted_index function and print the result
    match build_inverted_index(&args[1]) {
        Ok(result) => println!("{}", result),
        Err(err) => eprintln!("Failed to build inverted index: {}", err),
    }

    Ok(())
}

