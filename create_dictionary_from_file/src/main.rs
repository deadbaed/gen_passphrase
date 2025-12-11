use clap::Parser;
use std::fs::File;
use std::{
    io::{BufReader, Read, Write},
    path::PathBuf,
};

/// Create a dictionary from a text file containing words
#[derive(Parser, Debug)]
struct Args {
    /// Name of dictionary, used for feature name and file name
    name: String,

    /// Path of file containing words, one word per line. Leave empty to read from stdin
    file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let raw_input = match args.file {
        Some(file_path) => {
            println!(
                "Attempting to read words from file `{}`",
                file_path.display()
            );

            let file = File::open(file_path).expect("open file");
            let mut buf_reader = BufReader::new(file);
            let mut file_contents = String::new();
            buf_reader
                .read_to_string(&mut file_contents)
                .expect("read from file");
            file_contents
        }
        None => {
            println!("Attempting to read words from stdin");

            let mut file_contents = String::new();
            std::io::stdin()
                .read_to_string(&mut file_contents)
                .expect("read from stdin");

            file_contents
        }
    };

    let dictionary_name = args.name.to_lowercase();

    let mut new_dictionary = format!(
        "#[cfg(feature = \"{}\")]\npub static {}: &[&str] = &[\n",
        dictionary_name,
        dictionary_name.to_uppercase()
    );

    let mut count = 0;

    // Add words to the slice, following rustfmt defaults
    // Run rustfmt afterwards just in case
    for word in raw_input.lines() {
        new_dictionary.push_str("    \"");
        new_dictionary.push_str(word);
        new_dictionary.push_str("\",\n");

        count += 1;
    }

    // Finish slice
    new_dictionary.push_str("];\n");

    println!("Processed {count} words for dictionary `{dictionary_name}`");

    // Write new dictionary to file
    let output_path = format!("{dictionary_name}.rs");
    let mut output_file = File::create(&output_path).expect("file already exists");
    output_file
        .write_all(new_dictionary.as_bytes())
        .expect("write dictionary to file");

    println!("Wrote dictionary to `{output_path}`");
}
