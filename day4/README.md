Finishing Up Ray Tracing in a weekend

Splitting files into directories
https://doc.rust-lang.org/rust-by-example/mod/split.html

## Traits

## Vec

## Sized
http://huonw.github.io/blog/2015/01/the-sized-trait/


```
/Users/srushton/.cargo/bin/cargo check --color=always --all --all-targets
    Checking day4 v0.1.0 (/Users/srushton/projects/rushtonality/rust/my-tour-of-rust/wip/day4)
error[E0277]: the size for values of type `Hitable` cannot be known at compilation time
  --> src/raytrace/mod.rs:55:5
   |
55 |     list : LinkedList<Hitable>
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `Hitable`
   = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = help: consider adding a `where Hitable: std::marker::Sized` bound
   = note: required by `std::collections::LinkedList`
```


http://smallcultfollowing.com/babysteps/blog/2016/11/02/associated-type-constructors-part-1-basic-concepts-and-introduction/


https://stackoverflow.com/questions/25740916/how-do-you-actually-use-dynamically-sized-types-in-rust


```
/Users/srushton/.cargo/bin/cargo check --color=always --all --all-targets
    Checking day4 v0.1.0 (/Users/srushton/projects/rushtonality/rust/my-tour-of-rust/wip/day4)
error[E0658]: imports can only refer to extern crate names passed with `--extern` on stable channel (see issue #53130)
 --> src/main.rs:3:5
  |
1 | pub mod raytrace;
  | ----------------- not an extern crate passed with `--extern`
2 | 
3 | use raytrace::vec::Vec3;
  |     ^^^^^^^^
  |
note: this import refers to the module defined here
 --> src/main.rs:1:1
  |
1 | pub mod raytrace;
  | ^^^^^^^^^^^^^^^^^

error[E0658]: imports can only refer to extern crate names passed with `--extern` on stable channel (see issue #53130)
 --> src/main.rs:3:5
  |
1 | pub mod raytrace;
  | ----------------- not an extern crate passed with `--extern`
2 | 
3 | use raytrace::vec::Vec3;
  |     ^^^^^^^^
  |
note: this import refers to the module defined here
 --> src/main.rs:1:1
  |
1 | pub mod raytrace;
  | ^^^^^^^^^^^^^^^^^

warning: unused import: `raytrace::vec::Vec3`
 --> src/main.rs:3:5
  |
3 | use raytrace::vec::Vec3;
  |     ^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_imports)] on by default

warning: unused import: `raytrace::vec::Vec3`
 --> src/main.rs:3:5
  |
3 | use raytrace::vec::Vec3;
  |     ^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_imports)] on by default

error[E0061]: this function takes 2 parameters but 1 parameter was supplied
  --> src/main.rs:75:23
   |
20 | fn color(r : raytrace::ray::Ray, world : &raytrace::Hitable) -> raytrace::vec::Vec3 {
   | ----------------------------------------------------------------------------------- defined here
...
75 |             let col = color(r);
   |                       ^^^^^^^^ expected 2 parameters

error[E0061]: this function takes 2 parameters but 1 parameter was supplied
  --> src/main.rs:75:23
   |
20 | fn color(r : raytrace::ray::Ray, world : &raytrace::Hitable) -> raytrace::vec::Vec3 {
   | ----------------------------------------------------------------------------------- defined here
...
75 |             let col = color(r);
   |                       ^^^^^^^^ expected 2 parameters

error: aborting due to 2 previous errors

Some errors occurred: E0061, E0658.
For more information about an error, try `rustc --explain E0061`.
error: Could not compile `day4`.
warning: build failed, waiting for other jobs to finish...
error: aborting due to 2 previous errors

Some errors occurred: E0061, E0658.
For more information about an error, try `rustc --explain E0061`.
error: Could not compile `day4`.

To learn more, run the command again with --verbose.

Process finished with exit code 101 
```


Chapter 5 kicked my butt.  I had to circle
back around on heap vs stack, ownership, traits, etc.



The book uses the common C/C++ idea of passing an out parameter
of a reference and then using that to get the output of the
function.  I was doing a direct port of this in Rust and it
was working, until I got to chapter 8. When a pointer to 
material was added to the hit record.

At this point with Rust's ownership rules, I was having a hard
time to continue with this approach.

Changed this to return an Option with the Hit Record instead of
having it as an out paramater.


Box

Option










It has been two weeks since the last installment of my 2-3
post a week blog.  As I noted in an earlier post.  I am
committing to myself to spend at least an hour per day learning
new things and writing them up in this blog.  I decided that
I wanted to complete the "Ray Tracing in One Weekend" book for
this post.  I underestimated to new Rust concepts and conventions
that would be required to do this.  So now two weeks later, I am 
finally finished.

I have also made a decision to move on from Rust for a while and
pursue some other interests in up coming posting. I have found
myself really loving Rust.  I really appreciate all of the
compile time checks that can prevent problems later.  There were
only a couple of times that I got a good compile, that I did
not get the result I was expecting when I ran the program.

This exercise really forced me to understand Ownership and 
borrowing more thoroughly then.
  
## Splitting code into modules
My first objective in starting the rest of the chapters of
the book was to figure out how to structure a project with
multiple files and modules. 

The first thing that I did was start creating a separate
file to split different functionality.  That turned out being
fair straight forward.  It was just forward declaring a module
with the same name as the file from the the main.rs file.
```
pub mod raytrace;
```

Then I wanted to split things even more and create a tree structure
of directories.  So now I created a raytrace sub-directory, but I
not sure what to do next, because if I tried to do the mod statement
above, then all of my modules were not found.  Below is the file
structure that I had.

```
src/main.rs
src/raytrace
src/raytrace/ray.rs
src/raytrace/material.rs
src/raytrace/vec.rs
src/raytrace/camera.rs
```

Then I searched and found this page about [Splitting](https://doc.rust-lang.org/rust-by-example/mod/split.html).
So I saw that I needed a file mod.rs that would act like raytrace.rs
did in my first step.  Then in that file I could bring in the
other nested modules as below.
```
pub mod camera;
pub mod material;
pub mod ray;
pub mod vec;
```

## Traits and Polymorphism

## Ownership and Borrowing

### Moving vs Copying

### Out References

## Option returns to handle nulls

## External Crate



Like I said above.  This will likely be my last post on Rust
for a while.  I plan to branch off into some other technologies.
I hope someone has found this useful, because it has been
very valuable for me in forcing me to document what I have
been learning.  Thank you for taking time to read and follow me
on twitter [@rushtonality](https://twitter.com/rushtonality) for
future updates and anything else I my find interesting.
