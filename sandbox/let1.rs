fn main() {
    let answer = 42;
    println!("Hello {}", answer);
    assert_eq!(answer, 42);
    assert_eq!(answer, 40);
}