#[derive(Debug)]
struct ChatMessage {
    sender: String,
    content: String, 
    likes: u32,
}

fn create_message(sender: &str, content: &str) -> ChatMessage {
   ChatMessage{
    sender:sender.to_string(),
    content: content.to_string(),
    likes: 0,
   } 
}

fn has_forbidden_word(message: &ChatMessage, forbidden: &str) -> bool {
    message.content.contains(forbidden)
}

fn print_preview(message: &ChatMessage, max_chars: usize) {
    let preview: String = message.content.chars().take(max_chars).collect();
    println!("[Preview by {}]: {}...", message.sender, preview);

}

fn censor_word(content: &mut String, target: &str, replacement: &str) {
    let newContent = content.replace(target, replacement);
    *content = newContent;
}

fn main() {
    let mut msg1 = create_message("Ivan", "Hello, this is a very stupid chat!");
    let msg2 = create_message("Maria", "Rust is an amazing and safe programming language.");

    println!("Initial message 1: {:?}", msg1);

    if has_forbidden_word(&msg1, "stupid") {
        println!("Warning: Forbidden word detected in {}'s message!", msg1.sender);
    }

    print_preview(&msg1, 15);

    censor_word(&mut msg1.content, "stupid", "***");
    println!("After censorship msg1: {:?}", msg1);

}