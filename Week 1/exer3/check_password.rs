fn check_password(pass: &str) -> bool {
    if pass.len() >= 8
    {
        let first = pass.chars().next().unwrap();
        let last =pass.chars().last().unwrap();
        if first.is_uppercase() && last.is_numeric() {
            return true;
        }
    }
    false
}

fn main() -> ()
{
    println!("{}", check_password("Abcdefg1")); // true
    println!("{}", check_password("abcdefg1")); // false (не започва с главна)
    println!("{}", check_password("Abcdefgh")); // false (не завършва с цифра)
    println!("{}", check_password("Ab")); // false
}