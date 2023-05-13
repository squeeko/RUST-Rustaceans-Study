fn main() {
    let x1 = 42;
    let y1 = Box::new(84);
    { // starts a new scope
        let z = (x1, y1); // #1
        // "z" goes out of scope and is dropped
        // it in turn drops the values of "x1 and y1"
    } // #2
    // x1's is copy so it wasn't MOVED into "z"
    // conversely "y1" is NOT copied BUT is MOVED into "z"

    let x2 = x1; // #3

    dbg!(x1, x2);
    // dbg!(y1); - FAILS as it is not accessible because it was moved into "z" which 
    // is no longer in scope. Purposely being redundant here to grasp the conecpt
}

/*
*** OUTPUT ***

error[E0382]: use of moved value: `y1`
  --> src/main.rs:10:14
   |
3  |     let y1 = Box::new(84);
   |         -- move occurs because `y1` has type `Box<i32>`, which does not implement the `Copy` trait
4  |     {
5  |         let z = (x1, y1);
   |                      -- value moved here
...
10 |     dbg!(x1, y1, x2);
   |              ^^ value used here after move
   |
help: consider cloning the value if the performance cost is acceptable
   |
5  |         let z = (x1, y1.clone());
   |                        ++++++++
 */

/*
We start out with two values, the number 42 and a Box (a heap-allocated
value) containing the number 84. The former is Copy, whereas the latter is
not. When we place x1 and y1 into the tuple z 1, x1 is copied into z, whereas
y1 is moved into z. At this point, x1 continues to be accessible and can be
used again 3. On the other hand, y1 is rendered inaccessible once its value
has been moved 4, and any attempt to access it would incur a compiler
error. When z goes out of scope 2, the tuple value it contains is dropped,
and this in turn drops the value copied from x1 and the one moved from y1.
When the Box from y1 is dropped, it also deallocates the heap memory used
to store y1’s value.
 */