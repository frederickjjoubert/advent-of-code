use std::collections::HashMap;
use std::error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const FILE_PATH: &str = "./input_2.txt";

fn main() {
    println!("Part 2!");

    let mut trie = Trie::new();
    trie.insert("one".to_string());
    trie.insert("two".to_string());
    trie.insert("three".to_string());
    trie.insert("four".to_string());
    trie.insert("five".to_string());
    trie.insert("six".to_string());
    trie.insert("seven".to_string());
    trie.insert("eight".to_string());
    trie.insert("nine".to_string());

    println!("Contains 'seven': {}", trie.search("seven".to_string()));
    println!("Starts with 's': {}", trie.starts_with("s".to_string()));

    trie.print_all_words();

    // let result = part_1(FILE_PATH);
    // println!("Result: {}", result);
}

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }

    // Recursive function to collect and return all words from this node onwards
    fn collect_words(&self, prefix: String, words: &mut Vec<String>) {
        if self.is_end_of_word {
            words.push(prefix.clone());
        }
        for (&c, node) in self.children.iter() {
            let new_prefix = format!("{}{}", prefix, c);
            node.collect_words(new_prefix, words);
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::new);
        }
        node.is_end_of_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return false,
            }
        }
        node.is_end_of_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return false,
            }
        }
        true
    }

    // Method to print all words in the trie
    pub fn print_all_words(&self) {
        let mut words = Vec::new();
        self.root.collect_words(String::new(), &mut words);
        for word in words.iter() {
            println!("{}", word);
        }
    }
}

fn part_2(file_path: &str) -> u32 {
    let mut total: u32 = 0;
    match read_lines(file_path) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(line) => {
                        let mut value: String = String::from("");
                        println!("{}", line);
                        for c in line.chars() {
                            if c.is_digit(10) {
                                value.push(c);
                                break;
                            }
                        }
                        for c in line.chars().rev() {
                            if c.is_digit(10) {
                                value.push(c);
                                break;
                            }
                        }
                        match value.parse::<u32>() {
                            Ok(number) => {
                                println!("Parsed number: {}", number);
                                total += number;
                            }
                            Err(error) => {
                                println!("Failed to parse: {}", error);
                            }
                        }
                    }
                    Err(error) => {
                        println!("Failed to parse: {}", error);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }

    total
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_test() {
        let result = part_1(FILE_PATH);
        assert_eq!(result, 142);
    }
}
