
fn say_hello() {
    let fruits = ["香蕉", "葡萄", "西柚"];
    for item in fruits.iter() {
        println!("{}" ,item)
    }
}

fn main() {
    say_hello();
    println!("Hello, world!");
}
