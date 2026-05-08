fn main() {
    let a = String::from("everyone");
    let text: &String = &a;

    hellower(text);
    println!("{}", text);
}

fn hellower(text: &String) {
    let res: &String = text;
    println!("Hello, {}!", *res);
}
