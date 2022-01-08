use itertools::Itertools;
use std::cmp::{Ordering, Reverse};
use std::fs;

use pyo3::prelude::*;

fn contains_any_characters(word: &str, characters: Vec<char>) -> bool {
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

fn binary_search(word: &str, words: &[String]) -> bool {
    if words.len() <= 20 {
        return words.iter().any(|x| x == word);
    }

    let centre_index = (words.len() - 1) / 2;

    if word == words[centre_index] {
        return true;
    }

    match word.cmp(&words[centre_index]) {
        Ordering::Greater => binary_search(word, &words[centre_index..]),
        Ordering::Less => binary_search(word, &words[..centre_index]),
        _ => panic!(),
    }
}

fn all_lengths(anagram: &str, max: &usize, min: &usize) -> Vec<Vec<char>> {
    if *max <= *min {
        return anagram.chars().permutations(*max).unique().collect_vec();
    }

    let mut result: Vec<Vec<char>> = Vec::new();
    result.append(&mut anagram.chars().permutations(*max).unique().collect_vec());
    result.append(&mut all_lengths(anagram, &(max - 1), &min));

    result
}

#[pyfunction]
fn solve_anagram(anagram: &str, max: usize, min: usize) -> PyResult<Vec<String>>{
    let letters: Vec<Vec<char>> = all_lengths(&anagram, &max, &min);
    let words: Vec<String> = fs::read_to_string("words.txt")
        .expect("Couldn't open words.txt. Does it exist?")
        .split('\n')
        .map(String::from)
        .collect();

    let mut solved: Vec<String> = Vec::new();

    for perm in letters {
        let result = perm.into_iter().collect::<String>();

        if contains_any_characters(&result, vec!['a', 'e', 'i', 'o', 'y'])
            && !solved.iter().any(|x| x == &result)
            && binary_search(&result, &words)
        {
            solved.push(result);
        }
    }

    solved.sort_by_key(|a| Reverse(a.len()));
    Ok(solved)
}

#[pymodule]
fn anagram_solver(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_anagram, m)?)?;
    Ok(())
}
