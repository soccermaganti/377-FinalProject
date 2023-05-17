use std::collections::{BTreeMap, HashSet};
// use std::env;
use std::fs::File;
use std::io::{self, BufRead};
// use std::process;

fn print_set(s: &HashSet<usize>, out: &mut String) {
    for element in s.iter() {
        out.push_str(&format!(" {}", element));
    }
    out.push_str("\n");
}

fn index_inserter(word: &str, inverted_index: &mut BTreeMap<String, HashSet<usize>>, file_number: usize) {
    inverted_index
        .entry(word.to_string())
        .or_insert_with(HashSet::new)
        .insert(file_number);
}

pub fn build_inverted_index(filename: &str) -> io::Result<String> {
    let file_list_reader = io::BufReader::new(File::open(filename)?);

    // Creates a BTreeMap Data Structure
    let mut inverted_index: BTreeMap<String, HashSet<usize>> = BTreeMap::new();
    let mut file_number = 0;

    for file in file_list_reader.lines() {
        let cur_file_reader = io::BufReader::new(File::open(&file?)?);

        for line in cur_file_reader.lines() {
            let line = line?; 

            let mut word_start = 0;
    
             for (i, cur_char) in line.char_indices() {
                if !((cur_char >= 'a' && cur_char <= 'z') || cur_char >= 'A' && cur_char <= 'Z') {
                    if i == word_start {
                        word_start += 1;
                    } else {
                        let word = &line[word_start..i];
                        word_start = i + 1;
                        index_inserter(word, &mut inverted_index, file_number);
                    }
                }
            }
            
            if word_start != line.len() {
                let word = &line[word_start..];
                index_inserter(word, &mut inverted_index, file_number);
            }
        }

        file_number += 1;
    }

     let mut output = String::new();


     for word in inverted_index.keys() {
         let file_numbers = inverted_index.get(word).unwrap(); 
         output.push_str(&format!("{}:", word));
         print_set(&file_numbers, &mut output);
     }
     Ok(output)
}



