use algorithm;
fn main() {
    let mut vector = vec![1,6,45,2];
    algorithm::bubble_sort(&mut vector);
    println!("{:?}", vector);
}
