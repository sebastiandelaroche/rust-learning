fn main() {
    // let v = vec![1, 2, 3];
    // let v2 = Box::new(v);
    // println!("{:?}", v2);

    // let u = Box::new(1);
    // let u2 = u;
    // println!("{:?}", u2);

    let mut a = 40;
    {
        let b = &mut a;
        *b += 2;
    }

    println!("a = {}", a);
}
