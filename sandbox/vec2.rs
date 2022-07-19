// vec4.rs
fn main() {
    let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    println!("{:?}", v1);
    v1.sort();
    println!("sorted {:?}", v1);
    v1.dedup();
    println!("deduped {:?}", v1);
    assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);
}