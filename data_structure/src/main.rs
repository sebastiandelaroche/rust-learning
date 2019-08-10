#![allow(dead_code)]
#![allow(unused_variables)]
// use std::mem;
mod pm;

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

fn structure() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    let line = Line { start: p, end: p2 };
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn enums() {
    let c: Color = Color::CmykColor {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255,
    };

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) => println!("black rgb"),
        Color::CmykColor {
            cyan: _,
            magenta: 128,
            yellow: _,
            black: 255,
        } => println!("black cmyk"),
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        _ => (),
    }
}

fn option() {
    let x = 3.0;
    let y = 2.0;

    let result: Option<f64> = if y != 0.0 { Some(x / y) } else { None };

    println!("result is {:?}", result);

    match result {
        Some(z) => println!("{} / {} = {}", x, y, z),
        None => println!("cannot divide {} by {}", x, y),
    }

    if let Some(z) = result {
        println!("z = {}", z)
    };
}

fn array() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("a has {} elements, first is {}", a.len(), a[0]);

    a[0] = 321;

    println!("a[0] is {}", a[0]);

    println!("{:?}", a);

    let b = [1; 10];

    println!("b = {:?}", b);

    for x in 0..b.len() {
        println!("{}", b[x]);
    }
}

fn vector() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(12);

    println!("a = {:?}", a);

    let idx: usize = 1;

    println!("a[{}] = {}", idx, a[idx]);

    match a.get(6) {
        Some(x) => println!("a[3] = {}", x),
        None => println!("error, no such element"),
    }

    for x in &a {
        println!("{}", x);
    }

    a.push(44);
    println!("{:?}", a);

    let mut b = Vec::new();
    b.push(1);
    // let val_1 = b.pop();
    // let val_2 = b.pop();

    // println!("val_1 = {:?}, val_2 = {:?}", val_1, val_2);

    while let Some(x) = b.pop() {
        println!("{:?}", x);
    }
}

fn use_slice(slice: &mut [i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slice() {
    let mut data = [1, 2, 3, 4, 5];

    use_slice(&mut data[1..4]);
    // use_slice(&mut data);
    println!("{:?}", data);
}

fn string() {
    // utf-8
    let s: &'static str = "hello there!";
    // you cannot do something like this
    // s = "abc";
    // let h = s[0];

    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    }

    // heap
    // String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);

    // &str <> String
    let u: &str = &letters;

    let mut abc = "hello world".to_string();
    abc.push_str("!!!");
    abc.remove(0);
    println!("{}", abc.replace("ello", "goodbye"));
}

// this function return a tuple
fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c, d), (e, f)) = combined;

    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);
    let meaning = (42,);
    println!("{:?}", meaning);
}

struct Point2<T> {
    x: T,
    y: T,
}

struct Line2<T> {
    start: Point2<T>,
    end: Point2<T>,
}

fn generics() {
    let a: Point2<f32> = Point2 { x: 0.0, y: 0.0 };
    let b: Point2<f32> = Point2 { x: 1.2, y: 5.0 };

    let line = Line2 { start: a, end: b };

    // println!("{:?}", line);
}

fn main() {
    // structure();
    // enums();
    // option();
    // array();
    // vector();
    // slice();
    // string();
    // tuples();
    // pm::pattern_matching();
    generics();
}
