#![allow(dead_code)]
#![allow(unused_variables)]
mod sh;
// use std::mem;

// const PI: f64 = std::f64::consts::PI;
// static mut X: u32 = 123;

// fn variables() {
//     let a: u8 = 123; // 8bits
//     println!("a = {}", a);

//     let mut b: u8 = 0;
//     println!("b = {}", b);
//     b = 200;
//     println!("b = {}", b);

//     let c = 123456789; // 32-bit
//     println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
// }

// fn operators() {
//     let a = 14;
//     println!("{} % 3 = {}", a, (a % 3));

//     let a_cubed = i32::pow(a, 3);
//     println!("{} cubed is {}", a, a_cubed);

//     let b = 2.5;
//     let b_cubed = f64::powi(b, 3);
//     let b_to_pi = f64::powf(b, std::f64::consts::PI);
//     println!("{} cubed is {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

//     // bitwise
//     let c = 1 | 2;
//     println!("1 | 2 is {}", c);

//     let two_to_10 = 1 << 10;
//     println!("1 << 10 is {}", two_to_10);
// }

// fn scope_and_shadowing() {
//     let a = 123;

//     {
//         let b = 456;
//         println!("inside b = {}", b);
//         let a = 777;
//         println!("inside a = {}", a);
//     }

//     println!("outside a = {}", a);
// }

fn main() {
    // variables();
    // operators();
    // scope_and_shadowing();
    // println!("PI = {}", PI);

    // unsafe {
    //     println!("X = {}", X);
    // }

    sh::stack_and_heap();

    let number = 32;
    const x = &number;
    *x


}
