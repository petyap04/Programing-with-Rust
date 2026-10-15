use std::io;

fn check_triangle(a: i32, b: i32, c:i32) -> &'static str 
{
    if a < 0 || b < 0 || c < 0
    {
        "невалидни страни"
    }
    else
    {
        let sumAB = a + b;
        let sumAC = a + c;
        let sumBC = b + c;
        if sumAB <= c || sumAC <= b || sumBC <= a
        {
            "не е триъгълник"
        }
        else if a == b && b == c
        {
            "равностранен"
        }
        else if a == b || b == c || a == c
        {
            "равнобедрен"
        }
        else 
        {
            "разностранен"
        }
    }
}

fn main() -> ()
{
    println!("{}", check_triangle(1,1,1));
    println!("{}", check_triangle(1,2,3));
    println!("{}", check_triangle(3,4,5));
    println!("{}", check_triangle(3,3,5));
    println!("{}", check_triangle(4,7,9));
    println!("{}", check_triangle(-1,1,1));

    let mut input = String::new();
}