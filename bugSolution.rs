fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    // Create a copy to avoid borrow checker issues.
    let slice = vec.clone();
    for i in 0..slice.len() {
        println!("Value at index {}: {}", i, slice[i]);
    }
    // This is now safe as the original vector is not being borrowed
    vec.push(3);
    println!("Modified Vector: {:?}", vec);
} 