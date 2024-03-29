use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn increment(&mut self, word: &str) {
        let key = word.to_owned();
        let count = self.0.entry(key).or_insert(0);
        *count += 1
    }

    fn display(&self) {
        for (key, value) in &self.0 {
            println!("{}: {}", key, value)
        }
    }
}

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    let filename = &arguments[1];
    println!("Processing file: {}", filename);
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut word_counter = WordCounter::new();
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(" ");
        for word in words {
            if word == "" {
                continue;
            } else {
                word_counter.increment(word);
            }
        }
    }
    word_counter.display()
}
