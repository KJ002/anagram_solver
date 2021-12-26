use itertools::Itertools;
use reqwest;
use json;

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

fn main() {
    let perms = "test".split("").permutations("test".len());

    for perm in perms {
        let result = perm.join("").to_owned();

        if contains_any_characters(&result, vec!['a', 'e', 'i', 'o', 'y']) {
            let uri = "https://api.dictionaryapi.dev/api/v2/entries/en/".to_owned();
            let request = reqwest::blocking::get(format!("{}{}", uri, result))
                .unwrap();
            let status = request.status();
            let data: json::JsonValue = json::parse(request.text().unwrap().as_str()).unwrap();

            if status == 200{
                println!("{}", data[0]["word"])
            }
        }
    }
}
