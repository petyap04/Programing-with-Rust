fn sum_slice(arr: &[i32]) -> i32 {
    arr.iter().copied().sum()
}

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let s_full: &[i32] = &a;
    let s_part: &[i32] = &a[1..4];

    println!("Сума на целия масив: {}", sum_slice(s_full));
    println!("Сума на частичен срез: {}", sum_slice(s_part));

    let v = vec![10, 20, 30];
    println!("Сума на вектор: {}", sum_slice(&v));
}

// sum_slice(arr: &[i32]) събира всички елементи със iter().sum().
// &Vec<T> автоматично се превръща в &[T] (коерсия).
// &a[2] е &i32, не slice → трябва &a[2..3].