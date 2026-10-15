use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn prepend(&self, value: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: value,
                next: self.head.clone(), 
            })),
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref(); 
            &node.elem
        })
    }
}

// Bonus!!!

pub struct Arena<T> {
    owned: Vec<Rc<Node<T>>>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Arena { owned: Vec::new() }
    }

    pub fn prepend(&mut self, list: &List<T>, value: T) -> List<T> {
        let node = Rc::new(Node { elem: value, next: list.head.clone() });
        self.owned.push(node.clone());
        List { head: Some(node) }
    }
}

fn main() {
    let list = List::new().prepend(1).prepend(2).prepend(3);

    assert_eq!(list.head(), Some(&3));
    let tail = list.tail();
    assert_eq!(tail.head(), Some(&2));

    for x in list.iter() {
        println!("{x}");
    }
}