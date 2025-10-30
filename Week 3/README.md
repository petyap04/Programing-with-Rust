# Задачи по Rust: Structs, Enums и Pattern Matching

## 🧩 Задача 1: Система за проследяване на събития

Напиши програма, която дефинира тип `Event`, използвайки `enum`, който може да бъде една от следните разновидности:

- `Login { user: String, timestamp: u64 }`
- `Logout { user: String, timestamp: u64 }`
- `Purchase { user: String, item: String, amount: f64, timestamp: u64 }`
- `Error { code: i32, message: String, timestamp: u64 }`

Освен това, дефинирай структура `EventLog`, която съдържа **вектор** от събития:

```rust
struct EventLog {
    events: Vec<Event>,
}
```

### а) Добави метод `fn add_event(&mut self, event: Event)` към `EventLog`, който добавя събитието.

### б) Добави метод `fn user_spent(&self, user: &str) -> f64`, който връща общата сума, изхарчена от даден потребител.

### в) Добави метод `fn summaries_by_type(&self) -> HashMap<String, usize>`, който връща брой събития по тип.

### г) Добави функция `fn filter_events(&self, user: Option<&str>, after: Option<u64>) -> Vec<&Event>` — връща събития, отговарящи на зададените критерии чрез pattern matching.

---

## 🧠 Задача 2: Симулатор на превозни средства

Създай система за моделиране на различни видове превозни средства и поведението им при движение.

### а) Дефинирай:

```rust
enum VehicleKind {
    Car,
    Truck,
    Motorcycle,
    Bicycle,
}

struct Vehicle {
    kind: VehicleKind,
    fuel: f64,
    distance: f64,
}
```

### б) Имплементирай методи:

- `fn new(kind: VehicleKind, fuel: f64) -> Self`
- `fn drive(&mut self, km: f64) -> Result<(), String>` — изчислява разход според вида:
  - Car → 0.07 л/км
  - Truck → 0.15 л/км
  - Motorcycle → 0.05 л/км
  - Bicycle → 0.0 л/км

### в) Имплементирай метод `fn status(&self) -> String`, който връща описание на състоянието.

### г) Напиши функция `fn total_fuel_used(vehicles: &[Vehicle]) -> f64`, която изчислява общо изразходваното гориво чрез pattern matching.
