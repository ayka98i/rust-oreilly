#[derive(Copy, Clone)]
struct Label {
    number: u32,
}

fn main() {
    let l = Label { number: 3 };
    print(l);
    println!("My label number is: {}", l.number);
}

fn print(l: Label) {
    println!("STAMP: {}", l.number);
}
