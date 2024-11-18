use std::io;

fn main() {
    println!("Word and Character Counter");
    println!("Enter a sentence:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let sentence = input.trim(); // Remove leading/trailing whitespace

    let word_count = count_words(sentence);
    let char_count = count_characters(sentence);

    println!("Words: {}", word_count);
    println!("Characters (excluding spaces): {}", char_count);
}

fn count_words(sentence: &str) -> usize {
    sentence.split_whitespace().count()
}

fn count_characters(sentence: &str) -> usize {
    sentence.chars().filter(|&c| !c.is_whitespace()).count()
}
