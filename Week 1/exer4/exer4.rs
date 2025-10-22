fn main() {
    let value = 10;
    let value = value as f64 + 0.5;  // change of type from an integer to a floating-point number
    let value = format!("{}", value); // new type - string
}

// There are 3 variables with different types, but we cannot access the first two.
// This is possible thanks to "shadowing" in the language.