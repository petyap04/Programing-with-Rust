# 🦀 Rust Упражнения — Работа с владение, препратки и срезове

Този набор от задачи разглежда основни концепции в **Rust** като владение (*ownership*), заемане (*borrowing*), срезове (*slices*) и препратки (*references*).

---

## 🧩 Задача 1 — Дублиране на стойности

```rust
fn duplicate_i32(x: i32) -> (i32, i32) {
    // реализирай тази функция
}

fn duplicate_string(s: String) -> (String, String) {
    // реализирай тази функция
}

fn main() {
    let a = 10;
    let (a1, a2) = duplicate_i32(a);
    println!("a1 = {}, a2 = {}, original a = {}", a1, a2, a);

    let s = String::from("Здравей");
    let (s1, s2) = duplicate_string(s);
    println!("s1 = {}, s2 = {}", s1, s2);
}
```

### Указания:

- a) Имплементирай `duplicate_i32` и `duplicate_string`.
- b) Обясни защо в случая с `i32` е възможно да използваме `x` след извикване, но не можем да използваме `s` след `duplicate_string`.
- c) Какво става, ако `duplicate_string` върне `(s, s)` — защо компилаторът го забранява?

---

## 📜 Задача 2 — Препратки и мутабилност

```rust
fn print_str(s: &str) {
    println!("Стрингово съдържание: {}", s);
}

fn append_exclamation(s: &mut String) {
    // добави '!' в края на низа
}

fn main() {
    let mut s = String::from("Здравей");
    print_str(&s);
    append_exclamation(&mut s);
    print_str(&s);
}
```

### Указания:

- a) Попълни тялото на `append_exclamation`.
- b) Обясни защо използваме `&str`, а не `&String` във `fn print_str(s: &str)`.
- c) Обясни защо използваме `&mut String`, а не `&mut str` във `fn append_exclamation(s: &mut String)`.
- d) Какво би станало, ако в `main` направиш `let r1 = &s; let r2 = &mut s;` едновременно — ще компилира ли? Защо?

---

## ✂️ Задача 3 — Разделяне на низове

Имплементирай функции, които разделят низ на две части.  
Първият низ да съдържа първите `n` байта, вторият — останалите. Подаденото `n` винаги ще бъде валидно.

### Прототипи:

- `fn split_string_at(s: String, n: usize) -> (??, ??)`
- `fn split_slice_at(s: &str, n: usize) -> (??, ??)`

Опитай се да имплементираш и:

- `fn split_slice_mut_at(s: &mut str, n: usize) -> (??, ??)`

### Въпроси:

- Избери подходящ тип за резултата и обясни защо.
- Обясни как може или защо не може да се имплементира последната функция.

---

## 🔢 Задача 4 — Срезове и масиви

```rust
fn sum_slice(arr: &[i32]) -> i32 {
    // върни сумата на елементите в среза
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
```

### Указания:

- a) Имплементирай `sum_slice`.
- b) Обясни защо `&v` може да се използва като `&[i32]`.
- c) Какво ще стане, ако извикаш `sum_slice(&a[2])`?

---

## 💡 Задача 5 (Бонус) — Поправяне на владение и типове

Поправи функциите така, че програмата да се компилира и тестовете в `main` да минават **без промени в самия `main`**.

```rust
fn fill_vec(vec: Vec<i32>, num: i32) -> Vec<i32> {
    vec.push(num);
    vec
}
fn filled_vec(vec: Vec<i32>, num: i32) -> Vec<i32> {
    vec.push(num);
    vec
}

fn main() {
    let mut vec1 = vec![22, 44];
    fill_vec(&mut vec1, 66);
    fill_vec(&mut vec1, 88);
    assert_eq!(vec1, vec![22, 44, 66, 88]);

    let vec2 = filled_vec(filled_vec(vec![22, 44], 66), 88);
    assert_eq!(vec2, vec![22, 44, 66, 88]);
}
```

---

📘 **Цел на упражнението:**  
Да се затвърдят знанията за собственост (*ownership*), заемане (*borrowing*), мутабилност (*mutability*) и срезове (*slices*) в Rust.
