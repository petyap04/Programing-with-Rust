fn calc(a: i32, b: i32, op: char) -> i32 {
    if op == '+'{
        a + b
    }
    else if op == '-'{
        a - b
    }
    else if op == '*'{
        a * b
    }
    else if op == '/'{
        if b == 0 {
            0
        }
        else{
            a / b
        }
    }
    else{
        42
    }
}

fn main() -> ()
{
    println!("{}", calc(5, 3, '+')); 
    println!("{}", calc(5, 0, '/')); 
    println!("{}", calc(7, 2, '%'));
}