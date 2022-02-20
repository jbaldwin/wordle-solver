use std::collections::BTreeMap;
use std::collections::HashMap;

fn main() {
    let contents = std::fs::read_to_string("dictionary.txt").unwrap();
    let words: Vec<&str> = contents.split('\n').into_iter().collect();

    // Generate a map of letter -> count to score how common each letter is.
    // Each letter slot in the 5 letters gets its own count since a letter might
    // appear more in one slot than another.
    let mut counts: BTreeMap<char, [i64; 5]> = BTreeMap::new();

    for word in words.iter() {
        let mut pos: usize = 0;
        for letter in word.chars() {
            (*counts.entry(letter).or_insert([0,0,0,0,0]))[pos] += 1;
            pos += 1;
        }
    }

    let mut ordered: Vec<(char, [i64; 5])> = Vec::new();
    for (letter, count) in counts.iter() {
        ordered.push((*letter, count.clone()));
    }
    ordered.sort_by(|a, b| {
        let a_sum: i64 = a.1.iter().sum();
        let b_sum: i64 = b.1.iter().sum();
        
        b_sum.cmp(&a_sum)
    });

    for (letter, count) in ordered {
        println!("{} {:?}", letter, count);
    }
    println!();

    // Grade each word by its letters scores, note that duplicate letters that appear
    // in a word are negatively graded since it reduces the amount of letters checked
    // in each guess.
    let mut word_values: Vec<(&str, i64)> = Vec::new();

    for word in words {
        let mut duplicates: HashMap<char, i64> = HashMap::new();
        let mut value: i64 = 0;

        let mut pos: usize = 0;
        for letter in word.chars() {
            value += counts[&letter][pos];
            (*duplicates.entry(letter).or_insert(0)) += 1;

            pos += 1;
        }

        // If there are duplicate letters in a word, remove 1/5th of its value
        // for each pair of duplicates.
        let one_fifth: i64 = value / 5;
        for (_letter, dupe_count) in duplicates.iter() {
            if *dupe_count > 1 {
                value -= one_fifth;
            }
        }

        word_values.push((word, value));
    }

    word_values.sort_by(|a, b| b.1.cmp(&a.1));

    for (word, value) in word_values.iter() {
        // if *word == "later" || *word == "tears" {
            println!("{} {}", word, value);
        // }
    }

    // for (letter, count) in counts {
    //     println!("{} {}", letter, count);
    // }
}