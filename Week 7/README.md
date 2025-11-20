# Lifetimes в Rust

## 1. Какво са lifetimes?

В Rust **всяка референция трябва да бъде валидна** докато се използва.  
Lifetime-ът описва *обхвата*, в който дадена референция може да съществува безопасно.

Rust не пази lifetime информация по време на изпълнение — това са *само анотации*, които компилаторът използва за проверка.

---

## 2. Защо са нужни?

Функции, които връщат референции, трябва да уточняват **от кой аргумент произлизат** връщаните референции.

Примерен проблем:  

```rust
fn map_get(map: &HashMap<String, String>, key: &str) -> &str {
    &map[key]
}
```

Компилаторът не знае:

- дали върнатата референция идва от `map`
- или от `key`

→ води до грешка „missing lifetime specifier“.

---

## 3. Основен синтаксис

```rust
fn func<'a>(x: &'a str) -> &'a str { ... }
```

- `'a` е *lifetime параметър*
- Референцията `&'a str` е валидна поне толкова, колкото `'a`
- Функцията връща референция, която живее **колкото входа**

Lifetimes са подобни на generic параметрите:  
```rust
fn foo<T>(x: T) { ... }
fn bar<'a>(x: &'a str) { ... }
```

---

## 4. Примери

### **Пример 1 — връщане на референция**

```rust
fn trim<'a>(s: &'a str) -> &'a str {
    s.trim()
}
```

### **Пример 2 — връщане на по-дългия стринг**

```rust
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() { s1 } else { s2 }
}
```

Тук `'a` показва, че **резултатът живее колкото най-краткия от s1/s2**.

### **Пример 3 — различни lifetimes**

```rust
fn find<'a, 'b>(s: &'a str, needle: &'b str) -> Option<&'a str> {
    s.split_once(needle).map(|(left, _)| left)
}
```

---

## 5. Lifetime Elision — кога можем да пропускаме lifetimes

Rust има правила за автоматично извеждане на lifetimes:

1. Всеки входен параметър с референция получава свой lifetime.
2. Ако има само един входен lifetime → той се ползва и за резултата.
3. Ако първият аргумент е `&self` → резултатът се свързва с неговия lifetime.

Пример:

```rust
fn substr(s: &str, n: usize) -> &str
```

Автоматично се разширява до:

```rust
fn substr<'a>(s: &'a str, n: usize) -> &'a str
```

---

## 6. `'static` lifetime

`'static` означава референция, валидна през целия живот на програмата.

Пример:

```rust
let s: &'static str = "Hello";
```

Това важи за всички string литерали и статични стойности.

---

## 7. Lifetimes в структури

Когато структура съдържа референции, трябва да зададем lifetime:

```rust
struct Words<'a> {
    text: &'a str,
}
```

Метод:

```rust
impl<'a> Words<'a> {
    fn new(text: &'a str) -> Self { Self { text } }
}
```

---

## 8. Итеративен пример със структура

```rust
#[derive(Debug)]
struct Words<'a> {
    text: Option<&'a str>,
}

impl<'a> Words<'a> {
    fn new(text: &'a str) -> Self {
        Words { text: Some(text) }
    }

    fn next_word(&mut self) -> Option<&str> {
        let text = self.text?;
        match text.split_once(char::is_whitespace) {
            Some((w, rest)) => {
                self.text = Some(rest);
                Some(w)
            }
            None => {
                self.text = None;
                Some(text)
            }
        }
    }
}
```

---

## 9. Обобщение

- Lifetimes гарантират, че референциите са валидни.
- Не влияят на изпълнението — само на проверката.
- Нужни са главно в:
  - функции, връщащи референции
  - структури с референции
- Много случаи се покриват автоматично от *lifetime elision*.
- Без тях компилаторът не може да се увери, че референцията няма да „умре“ преди да я използваме.
