fn main() {
    let x = 5;
    let y = "6";
    let z = add(x, y);
    println!("Answer: {}", z);
    print_type_of(z);
}


fn add<T, U, V>(x: T, y: U) -> V where T: std::ops::Add<U, Output = V>,
{
    x + y
}