# Упражнение по Rust: Unit Testing и модулна организация

## 1. Unit Testing


### 1.1. Какво представлява Unit Testing?

Имаме програма, която се компилира успешно. Означава ли това, че тя работи правилно?

**Отговор:** Не. Компилаторът проверява синтаксис, типове и правила като ownership/borrowing, но не може да гарантира, че логиката реализира изискванията.

```rust
fn add(a: i32, b: i32) -> i32 {
    a - b
}
```

Кодът се компилира, но функцията изважда вместо да събира. Автоматизираните тестове откриват подобни логически грешки.

**Unit test (единичен тест)** проверява малка, обособена част от кода – обикновено функция или метод.

**Три стъпки на типичен тест – Arrange / Act / Assert**

1. Arrange – подготвяме входните данни.
1. Act – извикваме функцията.
1. Assert – сравняваме получения резултат с очаквания.

### 1.2. Първият тест в Rust

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add() {
    let result = add(2, 3);
    assert_eq!(result, 5);
}
```

- #[test] – атрибут, който маркира функцията като тест.
- fn test_add() – тестова функция, която Cargo може да изпълни.
- let result = add(2, 3) – извикваме тестваната функция.
- assert_eq!(result, 5) – проверяваме дали действителното е равно на очакваното.

Стартиране:

```bash
cargo test
```

Съкратен примерен изход:

```text
running 1 test
test test_add ... ok

test result: ok. 1 passed; 0 failed
```

Стандартният тест минава успешно, ако приключи без неуспешна проверка или неочакван panic. Не е необходимо да връща true/false.


### 1.3. Основните assert макроси

| Макрос | Какво проверява | Пример |
| --- | --- | --- |
| assert! | Условието е true | assert!(5 > 3); |
| assert_eq! | Две стойности са равни | assert_eq!(2 + 2, 4); |
| assert_ne! | Две стойности са различни | assert_ne!(3, 5); |

В Rust тези конструкции завършват с !, защото са макроси, а не обикновени функции.

```rust
#[test]
fn test_assertions() {
    assert!(10 > 5);
    assert_eq!(10 / 2, 5);
    assert_ne!(10, 5);
}
```

```rust
assert_eq!(result, 5, "Функцията върна грешен резултат");
```

Можем да добавим собствено съобщение към неуспешната проверка.


### 1.4. Какво е #[cfg(test)]?

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

- #[cfg(test)] – включва модула, когато програмата се компилира за тестване.
- mod tests – групира свързаните тестове в модул.
- use super::* – въвежда имената от родителския модул в обхвата на тестовия модул.

### 1.5. Демонстрация: откриваме грешка с тест

**Стъпка 1 – създаваме проект**

```bash
cargo new rust_gradebook --lib
cd rust_gradebook
```

Флагът --lib създава библиотечен проект, чийто основен файл е src/lib.rs.

**Стъпка 2 – поставяме умишлено сгрешена функция в src/lib.rs**

```rust
pub fn is_passing(score: u32) -> bool {
    score > 50
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_80_passes() {
        assert!(is_passing(80));
    }

    #[test]
    fn score_50_passes() {
        assert!(is_passing(50));
    }
}
```

**Стъпка 3 – стартираме тестовете**

```text
cargo test

# Съкратен очакван резултат:
running 2 tests
test tests::score_80_passes ... ok
test tests::score_50_passes ... FAILED

test result: FAILED. 1 passed; 1 failed
```

Проблемът е в граничната стойност 50 – използвали сме > вместо >=.

```rust
pub fn is_passing(score: u32) -> bool {
    score >= 50
}
```

> **Основен извод**
>
> Добрите тестове не проверяват само „типични“ стойности. Те трябва да включват и границите на условието – например 49, 50 и 51.


### 1.6. Практика

**Задача 1: резултат от изпит**

Дадена е функцията:

```rust
pub fn is_passing(score: u32) -> bool {
    score >= 50
}
```

Напишете три unit теста за резултати 49, 50 и 51 точки. Проверете какво връща функцията за всеки случай.

**Решение на задача 1**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_49_fails() {
        assert!(!is_passing(49));
    }

    #[test]
    fn score_50_passes() {
        assert!(is_passing(50));
    }

    #[test]
    fn score_51_passes() {
        assert!(is_passing(51));
    }
}
```

**Задача 2: среден резултат**

```rust
pub fn average(scores: &[u32]) -> Option<f64> {
    if scores.is_empty() {
        return None;
    }

    let sum: u32 = scores.iter().sum();
    Some(sum as f64 / scores.len() as f64)
}
```

- [40, 60, 80] → Some(60.0)
- [50] → Some(50.0)
- [] → None

**Решение на задача 2**

```rust
#[test]
fn average_three_scores() {
    assert_eq!(average(&[40, 60, 80]), Some(60.0));
}

#[test]
fn average_one_score() {
    assert_eq!(average(&[50]), Some(50.0));
}

#[test]
fn average_empty_scores() {
    assert_eq!(average(&[]), None);
}
```

Бележка: За прости точни стойности този пример с f64 е подходящ. При сложни изчисления с плаваща запетая директното сравнение може да бъде неподходящо.


### 1.7. Допълнителен материал при останало време

**Очакван panic**

```rust
fn divide(a: i32, b: i32) -> i32 {
    assert_ne!(b, 0, "Деление на нула");
    a / b
}

#[test]
#[should_panic(expected = "Деление на нула")]
fn test_divide_by_zero() {
    divide(10, 0);
}
```

По правило panic проваля теста. С #[should_panic] указваме, че това поведение се очаква. expected проверява дали съобщението съдържа зададения текст.

| Команда | Какво прави |
| --- | --- |
| cargo test | Стартира стандартните тестове |
| cargo test score_50 | Филтрира тестовете по име |
| cargo test -- --nocapture | Показва и изхода от println! при успешни тестове |
| cargo test -- --list | Показва наличните тестове |


## 2. Модулна организация


### 2.1. Защо разделяме проекта на модули?

Модулите разделят програмата на логически части. Например grading отговаря за изчисленията, а reports – за представянето на резултатите. Това улеснява четенето, тестването и поддръжката.


### 2.2. Package, Crate, Module

| Понятие | Значение |
| --- | --- |
| Package | Cargo пакет с Cargo.toml; организира един или повече crates. |
| Crate | Единица за компилация: библиотека или изпълнима програма. |
| Module | Логическа единица вътре в crate; организира код и контролира видимостта. |

Един package може да има src/lib.rs и src/main.rs. Това са корени на два различни crates от един и същ пакет.

**Примерна структура**

```text
rust_gradebook/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── grading.rs
│   └── reports.rs
└── tests/
    └── workflow.rs
```


### 2.3. Ключови думи: mod, pub, use, crate, super

**mod – дефинира/декларира модул**

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    println!("{}", math::add(2, 3));
}
```

math е модул; add е функция вътре в него; :: е синтаксис за път.

**pub – публичен достъп**

Елементите са частни по подразбиране. pub разрешава достъп извън модула, ако пътят до елемента също е достъпен. Без pub на функцията add примерът няма да се компилира.

**use – по-кратко използване на път**

```rust
use crate::math::add;

fn main() {
    println!("{}", add(2, 3));
}
```

Този фрагмент предполага, че math е дефиниран в същия crate. use не създава нов модул – само въвежда име в текущия обхват.

| Префикс | Значение |
| --- | --- |
| crate:: | Път, започващ от корена на текущия crate |
| super:: | Път към родителския модул |
| self:: | Път, започващ от текущия модул |


### 2.4. Демонстрация – разделяме проекта на файлове

**Стъпка 1: src/lib.rs**

```rust
pub mod grading;
pub mod reports;
```

Така декларираме двата публични модула; Rust ще намери кода им в src/grading.rs и src/reports.rs. Създаването на файл само по себе си не го включва автоматично в модулното дърво.

**Стъпка 2: src/grading.rs**

```rust
pub fn is_passing(score: u32) -> bool {
    score >= 50
}

pub fn average(scores: &[u32]) -> Option<f64> {
    if scores.is_empty() {
        return None;
    }

    let sum: u32 = scores.iter().sum();
    Some(sum as f64 / scores.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passing() {
        assert!(is_passing(50));
        assert!(!is_passing(49));
    }

    #[test]
    fn test_average() {
        assert_eq!(average(&[40, 60, 80]), Some(60.0));
    }

    #[test]
    fn test_empty_average() {
        assert_eq!(average(&[]), None);
    }
}
```

Тестовете могат да стоят близо до кода, който проверяват. В този случай модулът tests е вложен в grading.

**Стъпка 3: src/reports.rs**

```rust
use crate::grading::is_passing;

pub fn status_message(score: u32) -> &'static str {
    if is_passing(score) {
        "Успешно"
    } else {
        "Неуспешно"
    }
}
```

reports използва grading, вместо да дублира логиката score >= 50. Ако критерият се промени, поддържаме един източник на истината.

**Стъпка 4: src/main.rs**

```rust
use rust_gradebook::{grading, reports};

fn main() {
    let scores = [40, 60, 80];

    let avg = grading::average(&scores);
    println!("Среден резултат: {:?}", avg);

    let status = reports::status_message(50);
    println!("Резултат: {}", status);
}
```

Името rust_gradebook идва от Cargo.toml. В main.rs използваме библиотеката по нейното име, защото main.rs и lib.rs са отделни crate корени.

```text
cargo run

# Очакван изход:
Среден резултат: Some(60.0)
Резултат: Успешно
```


### 2.5. Unit Tests и Integration Tests

| Критерий | Unit test | Integration test |
| --- | --- | --- |
| Проверява | Отделна функция/модул | Съвместна работа между компоненти |
| Обичайно място | В src/, в #[cfg(test)] модул | В директория tests/ |
| Достъп до private | Възможен при подходящо влагане | Не – използва публичния API |
| Пример | Проверка на average() | Изчисление + статус на студента |

Файловете в tests/ се компилират като отделни тестови crates. Те използват библиотеката така, както би я използвала външна програма.

**src/ и tests/ в един реален сценарий**

```rust
// tests/workflow.rs
use rust_gradebook::{grading, reports};

#[test]
fn test_student_result() {
    let scores = [40, 60, 80];
    let average = grading::average(&scores).unwrap();

    assert_eq!(average, 60.0);

    let status = reports::status_message(average as u32);
    assert_eq!(status, "Успешно");
}
```

Сценарий: scores → grading::average() → reports::status_message() → проверка на крайния резултат.

```bash
cargo test
cargo test --test workflow
```

### 2.6. Самостоятелна задача – разширяване на проекта

В rust_gradebook добавете допълнителна функционалност, без да дублирате съществуващата логика.

**Част А – Отличен резултат**

```rust
pub fn is_excellent(score: u32) -> bool
```

В модула grading функцията връща true за score >= 90. Напишете unit тестове за 89, 90 и 100 точки.

**Част Б – Детайлно съобщение**

```rust
pub fn detailed_status(score: u32) -> &'static str
```

| Резултат | Съобщение |
| --- | --- |
| 0–49 | Неуспешно |
| 50–89 | Успешно |
| 90 и повече | Отлично |

В reports използвайте is_passing() и is_excellent() вместо да дублирате числовите условия.

**Част В – Тестове**

Добавете най-малко три unit теста за detailed_status и един integration test през публичния API на библиотеката.


### 2.7. Решения на задачите от втория час

**src/grading.rs – добавя се функция и тестове**

```rust
pub fn is_excellent(score: u32) -> bool {
    score >= 90
}

#[cfg(test)]
mod excellent_tests {
    use super::*;

    #[test]
    fn test_excellent() {
        assert!(!is_excellent(89));
        assert!(is_excellent(90));
        assert!(is_excellent(100));
    }
}
```

**src/reports.rs – добавя се импорт и нова функция**

```rust
use crate::grading::is_excellent;

pub fn detailed_status(score: u32) -> &'static str {
    if is_excellent(score) {
        "Отлично"
    } else if is_passing(score) {
        "Успешно"
    } else {
        "Неуспешно"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detailed_status() {
        assert_eq!(detailed_status(49), "Неуспешно");
        assert_eq!(detailed_status(50), "Успешно");
        assert_eq!(detailed_status(90), "Отлично");
    }
}
```

Забележка: горният откъс се добавя към вече съществуващия src/reports.rs, където is_passing е импортирана функция.

**tests/workflow.rs – добавя се integration test**

```rust
#[test]
fn test_excellent_report() {
    assert!(grading::is_excellent(95));
    assert_eq!(reports::detailed_status(95), "Отлично");
}
```


## 3. Често задавани въпроси и грешки

**Защо тестът не се изпълнява с cargo run?**

cargo run стартира изпълнимата програма, а cargo test компилира и изпълнява тестовете.

**Мога ли да имам unit test без #[cfg(test)]?**

Да. #[test] маркира тестова функция, докато #[cfg(test)] обичайно се използва за изключване на целия тестов модул от нормалната компилация.

**Защо компилаторът не намира функцията в друг модул?**

Провери mod, правилния път, pub върху необходимите елементи и дали use е написан за правилния crate.

**Какво означава super::*?**

Въвежда имената от родителския модул; вложените unit тестове могат да проверяват и частни функции на родителския модул.

**Ако всички тестове минават, значи ли, че няма бъгове?**

Не. Тестовете проверяват само случаите и свойствата, които сме описали.

**Всеки модул ли трябва да има свой файл?**

Не. Може да дефинираме mod name { ... } в същия файл; разделянето е организационно решение.


## 4. Финална проверка – въпроси към студентите

**1. Каква е целта на #[test]?**

- а) Дефинира нов модул
- б) Отбелязва функция като тест
- в) Прави функцията публична

**2. Кой макрос сравнява две стойности за равенство?**

- а) assert_eq!
- б) assert_ne!
- в) println!

**3. За какво служи pub?**

- а) Стартира компилацията
- б) Създава crate
- в) Управлява публичната видимост

**4. Какво прави use?**

- а) Създава файл
- б) Въвежда име/път в обхвата
- в) Компилира тестовете

**5. Къде обичайно се поставят интеграционните тестове?**

- а) В Cargo.toml
- б) В target/
- в) В tests/

**Ключ с отговори (за преподавателя)**

1 – б; 2 – а; 3 – в; 4 – б; 5 – в.
