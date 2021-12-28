use itertools::Itertools;
use std::cmp::{Ordering, Reverse};
use std::fs;

fn contains_any_characters(word: &String, characters: Vec<char>) -> bool {
    for character in characters {
        if word
            .to_lowercase()
            .contains(&character.to_lowercase().to_string())
        {
            return true;
        }
    }
    false
}

fn binary_search(word: &String, words: &[String]) -> bool {
    if words.len() <= 20 {
        for word_ in words {
            if word_ == word {
                return true;
            }
        }

        return false;
    }

    let centre_index = (words.len() - 1) / 2;

    if word == &words[centre_index] {
        return true;
    }

    match word.cmp(&words[centre_index]) {
        Ordering::Greater => return binary_search(word, &words[centre_index..]),
        Ordering::Less => return binary_search(word, &words[..centre_index]),
        _ => panic!(),
    }
}

fn main() {
    let perms = "this".split("").permutations("this".len());
    let words: Vec<String> = fs::read_to_string("words.txt")
        .expect("Couldn't open words.txt. Does it exist?")
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    let mut solved: Vec<String> = Vec::new();

    for perm in perms {
        let result = perm.join("").to_owned();

        if contains_any_characters(&result, vec!['a', 'e', 'i', 'o', 'y'])
            && !solved.iter().any(|x| x == &result)
            && binary_search(&result, &words)
        {
            solved.push(result);
        }
    }

    solved.sort_by_key(|a| Reverse(a.len()));

    for solved_word in solved {
        println!("{}", solved_word);
    }
}
