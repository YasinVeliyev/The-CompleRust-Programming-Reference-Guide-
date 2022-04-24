use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    fn new() -> Self {
        WordCounter(HashMap::new())
    }

    fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1
    }

    fn display(self) {
        println!("{:#?}", self.0);
    }
}

pub fn call_word_counter(filename: &str) {
    println!("Processing file :{}", filename);

    let reader = match File::open(filename) {
        Ok(file) => BufReader::new(file),
        Err(_) => panic!("Could not open file"),
    };
    // let file = File::open(filename).expect("Could not open file");
    // let reader = BufReader::new(file);

    let mut word_counter = WordCounter::new();

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => panic!("Could not read line"),
        };

        let words = line.split(" ");
        for word in words {
            if word == "" {
                continue;
            } else {
                word_counter.increment(word);
            }
        }
    }
    word_counter.display();
}
