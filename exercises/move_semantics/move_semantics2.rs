// move_semantics2.rs
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// Expected output:
// vec0 has length 3 content `[22, 44, 66]`
// vec1 has length 4 content `[22, 44, 66, 88]`


fn main() {
    let mut vec0 = Vec::new();

    fill_vec(&mut vec0);  // 修改 `vec0` 但不移动所有权

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    let mut vec1 = vec0;  // 现在移动所有权到 `vec1`

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}