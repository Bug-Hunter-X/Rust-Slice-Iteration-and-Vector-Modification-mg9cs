fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let slice = &vec[..];
    for i in 0..slice.len() {
        println!("Value at index {}: {}", i, slice[i]);
    }
    //Modifying vector here causes runtime error.
    vec.push(3);
}