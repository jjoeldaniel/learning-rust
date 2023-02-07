fn main() {
    let words = String::from("word1 word2 word3");

    let first_word = first_word(&words);
    println!("{first_word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; 
        }
    }

    &s[..]
}
