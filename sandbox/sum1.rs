fn main() {
    let sum: i32 = (0..10).sum();
    println!("sum {}", sum);

    let sum: i64 = [10,20,30].iter().sum();
    println!("sum {}", sum);

    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;

    for s in slice.windows(2) {
        println!("window {:?}", s);
    }

    for s in slice.chunks(2) {
        println!("chunks {:?}", s);
    }
}