#[derive(Debug)]
enum Spec {
    SI,
    IS,
    KN,
    I,
    M,
}

impl Spec {
    fn to_json(&self) -> String {
        match self {
            Spec::SI => "\"SI\"".to_string(),
            Spec::IS => "\"IS\"".to_string(),
            Spec::KN => "\"KN\"".to_string(),
            Spec::I  => "\"I\"".to_string(),
            Spec::M  => "\"M\"".to_string(),
        }
    }
}

#[derive(Debug)]
enum Title {
    Assistant,
    Doctor,
    Professor,
}

impl Title {
    fn to_json(&self) -> String {
        match self {
            Title::Assistant => "\"Assistant\"".to_string(),
            Title::Doctor => "\"Doctor\"".to_string(),
            Title::Professor => "\"Professor\"".to_string(),
        }
    }
}

#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
    spec: Spec,
}

impl Student {
    fn new(name: String, age: u32, spec: Spec) -> Student {
        Student { name, age, spec }
    }

    fn to_json(&self) -> String {
        format!(
            "{{\"name\": \"{}\", \"age\": {}, \"spec\": {}}}",
            self.name,
            self.age,
            self.spec.to_json()
        )
    }
}

#[derive(Debug)]
struct Teacher {
    name: String,
    age: u32,
    spec: Vec<Spec>,
    title: Title,
}

impl Teacher {
    fn new(name: String, age: u32, spec: Vec<Spec>, title: Title) -> Teacher {
        Teacher { name, age, spec, title }
    }

    fn to_json(&self) -> String {
        let specs_json: Vec<String> = self.spec.iter().map(|s| s.to_json()).collect();
        format!(
            "{{\"name\": \"{}\", \"age\": {}, \"spec\": [{}], \"title\": {}}}",
            self.name,
            self.age,
            specs_json.join(", "),
            self.title.to_json()
        )
    }
}

#[derive(Debug)]
struct University<T, U> {
    name: String,
    students: Vec<T>,
    teachers: Vec<U>,
}

impl<T, U> University<T, U> 
where
    T: StudentToJson,
    U: TeacherToJson,
{
    fn new(name: String, students: Vec<T>, teachers: Vec<U>) -> University<T, U> {
        University { name, students, teachers }
    }

    fn to_json(&self) -> String {
        let students_json: Vec<String> = self.students.iter().map(|s| s.to_json()).collect();
        let teachers_json: Vec<String> = self.teachers.iter().map(|t| t.to_json()).collect();

        format!(
            "{{\"name\": \"{}\", \"students\": [{}], \"teachers\": [{}]}}",
            self.name,
            students_json.join(", "),
            teachers_json.join(", ")
        )
    }
}

trait StudentToJson {
    fn to_json(&self) -> String;
}

trait TeacherToJson {
    fn to_json(&self) -> String;
}

impl StudentToJson for Student {
    fn to_json(&self) -> String {
        Student::to_json(self)
    }
}

impl TeacherToJson for Teacher {
    fn to_json(&self) -> String {
        Teacher::to_json(self)
    }
}

fn main() {
    let s1 = Student::new("Ivan".to_string(), 20, Spec::SI);
    let t1 = Teacher::new(
        "Dr. Petrov".to_string(),
        45,
        vec![Spec::SI, Spec::IS],
        Title::Professor,
    );

    let uni = University::new("SU".to_string(), vec![s1], vec![t1]);
    println!("{}", uni.to_json());
}