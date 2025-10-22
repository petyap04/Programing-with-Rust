use std::io;

fn fn1() {
   println!("Въведи число:");

   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Грешка при четене");

   let number: i32 = input.trim().parse().expect("Моля, въведи валидно число!");

   println!("Въведе числото: {}", number);
}

fn fn2() {
    println!("Въведи името си:");

    let mut name = String::new(); // създаваме празен String

    io::stdin()
        .read_line(&mut name) // четем ред от конзолата
        .expect("Грешка при четене от конзолата");

    println!("Здравей, {}!", name.trim()); // .trim() маха новия ред \n
}

fn main(){
   fn1();
   fn2();
}