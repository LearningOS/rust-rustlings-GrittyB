// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.


fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0); // 这里把vec0传入了,所有权转移, 因为没有Copy, 之后也不能使用vec0了

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // 这里不是产生了一个新的vec, 只是所有权转移了, 旧的vec的所有权消失
    let mut vec = vec; 

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
