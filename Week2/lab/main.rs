struct User {
    name: String,
    age: u8,
}

fn print_user(user: User) {
    println("{:?}", user);
}

fn main() {
    let u1 = 123;
    let u2 = u1;

    println!("{}", u1);
    println!("{}", u2);
}