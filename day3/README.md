# My Tour of Rust

## Ray Tracing in a Weekend

I wanted to provide more structure to my learning, so I decided
to work on an actual project.  I follow Peter Shirley on twitter [@Peter_shirley](https://twitter.com/Peter_shirley),
and I have been meaning to go through his book [Ray Tracing in a Weekend](https://github.com/petershirley/raytracinginoneweekend).
So I decided to kill two birds with one stone and use that a structure
to learning some Rust.




Chapter 1

Write simple program that prints data for a PPM formatted file

http://netpbm.sourceforge.net/doc/ppm.html


On OSX, the file is viewable by default, but on Windows, you will need
a program or you can use this online viewer.

http://paulcuth.me.uk/netpbm-viewer/

Really, the main thing to see here is the use of casting in Rust using the "as"
notation.  This file just writes to stdout, so you just need to redirect that
to a file to see the results.

```
fn main() {
    let nx = 200;
    let ny = 100;

    print!("P3\n{} {}\n255\n", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b :f32 = 0.2;
            let ir = (255.00*r) as i32;
            let ig = (255.00*g) as i32;
            let ib = (255.00*b) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
```

```
cargo run > image.ppm
```


Chapter 2

In this chapter we create a 3D Vector structure.  The actual class in the book is written in C++ using main concepts that I do not know yet
in Rust, so this should be a good time to learn.

Struct
```
pub struct Vec3 {
    e : [f32; 3],
}
```

Methods
```
impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }
}
```

Adding builtin traits
```
#[derive(Copy, Clone, Debug)]
```

Custom Constructors

Traits

Operator Overloading

No function overloading, how can this work
https://internals.rust-lang.org/t/justification-for-rust-not-supporting-function-overloading-directly/7012

https://blog.rust-lang.org/2015/05/11/traits.html

Ownership and Borrowing

Implementing std::fmt::Display





```
cargo test --no-run --package rtinw_ch2 --bin rtinw_ch2 tests
   Compiling rtinw_ch2 v0.1.0 (/my-tour-of-rust/day3/rtinw_ch2)
error[E0382]: borrow of moved value: `v`
  --> src\main.rs:43:13
   |
43 |         v / v.length()
   |         -   ^ value borrowed here after move
   |         |
   |         value moved here
   |
   = note: move occurs because `v` has type `Vec3`, which does not implement the `Copy` trait

```   