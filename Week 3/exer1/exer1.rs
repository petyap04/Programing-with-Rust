fn rainbow_replace(text: &str, word: &str, replacement: &[char]) -> String {
    if word.is_empty() || replacement.is_empty() {
        return text.to_string();
    }

    let mut result = String::with_capacity(text.len());
    let mut remaining = text;

    while let Some(pos) = remaining.find(word) {
        let (before, after_with_word) = remaining.split_at(pos);
        result.push_str(before);

        for (i, _ch) in word.chars().enumerate() {
            let rep_char = replacement[i % replacement.len()];
            result.push(rep_char);
        }

        remaining = &after_with_word[word.len()..];
    }

    result.push_str(remaining);
    result
}

fn main(){
    let replaced = rainbow_replace("плодова салата", "плодова", &['🍆', '🍎', '🍒']);
    println!("Replaced: {}", replaced);
}