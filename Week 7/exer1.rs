struct Map<I, F> {
    iter: I,
    f: F,
}

fn iter_map<I, F, U>(iter: I, f: F) -> Map<I, F>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    Map { iter, f }
}

impl<I, F, U> Iterator for Map<I, F>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&self.f)
    }
}

struct Filter<I, F> {
    iter: I,
    pred: F,
}

fn iter_filter<I, F>(iter: I, func: F) -> Filter<I, F>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    Filter { iter, pred: func }
}

impl<I, F> Iterator for Filter<I, F>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.iter.next() {
            if (self.pred)(&x) {
                return Some(x);
            }
        }
        None
    }
}

fn iter_filter_map<I, F, U>(iter: I, func: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> Option<U> + Copy,
{
    iter_map(
        iter_filter(iter_map(iter, func), |x| x.is_some()),
        |x| x.unwrap(),
    )
}

fn get_sum(text: &str) -> i32 {
    text.lines()
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .sum()
}

#[derive(Debug, Eq, PartialEq)]
struct FoundWord<'a>(&'a str);

fn get_num_list(text: &str) -> Result<Vec<i32>, FoundWord<'_>> {
    text.lines()
        .map(|line| {
            let t = line.trim();
            if t.is_empty() {
                Ok(0).map(|_| None)
            } else if let Ok(n) = t.parse::<i32>() {
                Ok(Some(n))
            } else {
                Err(FoundWord(t))
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|v| v.into_iter().flatten().collect())
}

fn main() {
    let txt = " 12\n   5\nhello\n";
    println!("{:?}", get_sum(txt));         // 17
    println!("{:?}", get_num_list(txt));    // Err(FoundWord("hello"))
}