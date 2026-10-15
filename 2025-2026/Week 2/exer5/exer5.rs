fn fill_vec(vec: &mut Vec<i32>, num: i32) {
    vec.push(num);
}
fn filled_vec(vec: Vec<i32>, num: i32) -> Vec<i32> {
    let mut vec = vec;  // !!
    vec.push(num);
    vec
}
fn filled_vec_2(mut vec: Vec<i32>, num: i32) -> Vec<i32> {
            //  ^^^  !!
    vec.push(num);
    vec
}