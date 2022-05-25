fn main() {
    println!("Hello, world!");
    println!("I guess this works which is pretty nice");
    println!("{}, {}, {}!", "Hello", "you absolute", "child");

    let a = format!("{} {} {} {}", "This", "was", "a", "triumph");
    println!("{}", a);

    println!("{:?}", [1, 2, 3, 4, 5]);
    println!("{:#?}", [1, 2, 3, 4, 5]);
}
