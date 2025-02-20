fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using a raw pointer, use safe methods to modify the vector.
    v[0] = 10; 
    println!("The first element is: {}", v[0]);
} 