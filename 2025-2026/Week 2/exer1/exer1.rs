fn duplicate_i32(x: i32) -> (i32, i32) {
    (x, x)
}

fn duplicate_string(s: String) -> (String, String) {
    (s.clone(), s) // клонираме първата стойност и след това местим original-а
}

fn main() {
    let a = 10;
    let (a1, a2) = duplicate_i32(a);
    println!("a1 = {}, a2 = {}, original a = {}", a1, a2, a);

    let s = String::from("Здравей");
    let (s1, s2) = duplicate_string(s);
    println!("s1 = {}, s2 = {}", s1, s2);
}


//Обяснения:
//i32 е Copy → може да се ползва след предаване.
//String не е Copy, а Move → може само веднъж да се премести.
//Решение: clone() копира съдържанието (heap данни), така може (s.clone(), s).