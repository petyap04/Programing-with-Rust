use std::io;

fn main() {
   println!("Въведи число:");

   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Грешка при четене");

   let number: i32 = input.trim().parse().expect("Моля, въведи валидно число!");

   println!("Въведе числото: {}", number);
}

