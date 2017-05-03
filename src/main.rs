use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::BufReader;
use std::io::BufRead;
use std::env::current_dir;
use std::collections::{BTreeMap, HashMap};

fn get_anagram_key(trimmed: &String) -> Option<String> {
    if trimmed.starts_with("#") || trimmed.len() == 0 {
        return None;
    }

    println!("[{}]", trimmed);
    let mut char_histo = BTreeMap::new();
    for c in trimmed.chars() {
        let new_val = match char_histo.get(&c) {
            Some(old_count) => old_count+1,
            None => 1,
        };
        char_histo.insert(c, new_val);
    }
    let mut anagram_key = String::new();
    for (c, n) in &char_histo {
        anagram_key = format!("{}{}{}", anagram_key, c, n);
    }
    return Some(anagram_key);
}

fn main() {
    let mut abspath = match current_dir() {
        Err(why) => panic!("Failed to find current dir {}", &why),
        Ok(x) => x,
    };
    abspath.push("word.lst");

    println!("Word list file abspath: {}", abspath.display());

    if !abspath.exists() {
        panic!("Word list file {} does not exist. Aborting.", abspath.display());
    }
    let path = Path::new("word.lst");
    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", path.display(),
            Error::description(&why)),
        Ok(f) => f,
    };

    let b = BufReader::new(file);

    let mut anagrams: HashMap<String, Box<Vec<String>>> = HashMap::new();
    for line in b.lines() {
        let trimmed = String::from(line.unwrap().trim());

        if let Some(anagram_key) = get_anagram_key(&trimmed) {
            println!("Anagram key: {:?}", anagram_key);

            if !anagrams.contains_key(&anagram_key) {
                let b = Box::<Vec<String>>::new(Vec::<String>::new());
                anagrams.insert(anagram_key.clone(), b);
            }

            if let Some(r) = anagrams.get_mut(&anagram_key) {
                r.push(String::from(trimmed));
            }
        }
    }
    for (k, v) in &anagrams {
        println!("{}: {:?}", k, v);
    }
}
