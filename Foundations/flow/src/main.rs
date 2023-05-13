fn main() {
    let mut x;
    x = 42; // #1
    let y = &x; // #2
    x = 43; // #3
    
    // assert_eq!(*y, 42); // #4

    dbg!(x);
}

/*
*** OUTPUT ***

error[E0506]: cannot assign to `x` because it is borrowed
 --> src/main.rs:5:5
  |
4 |     let y = &x; // #2
  |             -- `x` is borrowed here
5 |     x = 43; // #3
  |     ^^^^^^ `x` is assigned to here but it was already borrowed
6 |     
7 |     assert_eq!(*y, 42); // #4
  |     ------------------ borrow later used here
 */

/*
First, we cannot use x before it is initialized, because we have nowhere
to draw the flow from. Only when we assign a value to x can we draw flows
from it. This code has two flows: one exclusive (&mut) flow from 1 to 3, and
one shared (&) flow from 1 through 2 to 4. The borrow checker inspects
every vertex of every flow and checks that no other incompatible flows exist
concurrently. In this case, when the borrow checker inspects the exclusive
flow at 3, it sees the shared flow that terminates at 4. Since you cannot
have an exclusive and a shared use of a value at the same time, the borrow
checker (correctly) rejects the code. Notice that if 4 was not there, this
code would compile fine! The shared flow would terminate at 2, and when
the exclusive flow is checked at 3, no conflicting flows would exist.
If a new variable is declared with the same name as a previous one, they
are still considered distinct variables. This is called shadowing—the later
variable “shadows” the former by the same name. The two variables coexist,
though subsequent code no longer has a way to name the earlier one. This
model matches roughly how the compiler, and the borrow checker in particular,
reasons about your program, and is actually used internally in the
compiler to produce efficient code.

 */