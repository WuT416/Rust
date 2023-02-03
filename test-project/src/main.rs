
fn say_hello() {
    let fruits = ["香蕉", "葡萄", "西柚"];
    for item in fruits.iter() {
        println!("我喜欢的水果有{}。" ,item)
    }
}

fn main() {
    say_hello();
    println!("Hello, world!");
}
