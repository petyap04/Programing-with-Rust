fn split_at(s: String, n: usize) -> (String, String) {
    let (left, right) = s.split_at(n);
    (left.to_string(), right.to_string())
}

//fn split_at(s: &str, n: usize) -> (&str, &str) {
 //   s.split_at(n)
//}

fn main() {
    let s = String::from("Аз съм Иван");
    let res = split_at(s,7);
    println!("{:?}", res);
}

//Функция с &mut str - Не може да се имплементира директно:
//str е slice тип, не може да се раздели на две независими mutable части, освен чрез unsafe код.
//Safe Rust не позволява две &mut към части от един и същи обект (би нарушило borrow правилата).