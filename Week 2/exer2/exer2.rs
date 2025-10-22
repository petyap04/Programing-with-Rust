fn print_str(s: &str) {
    println!("Стрингово съдържание: {}", s);
}

fn append_exclamation(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut s = String::from("Здравей");
    print_str(&s);
    append_exclamation(&mut s);
    print_str(&s);
}

//print_str(&str) — само чете → &str е по-гъвкав от &String.
//append_exclamation(&mut String) — променя → нужен е String, защото str няма собствена памет.
//Не може &s и &mut s едновременно → нарушава borrow правилата 
//(няма едновременен достъп за четене и писане).