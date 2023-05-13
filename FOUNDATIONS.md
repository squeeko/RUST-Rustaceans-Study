Ch 1 - Foundations

Value - The combination
of a type and an element of that type’s domain of values. A value can
be turned into a sequence of bytes using its type’s representation. A value 
is stored in a "place" which is a location that can hold a value. The "place"
can be the Stack, Heap or other Memory Locations. Values are commonly stored in
a "variable" which is a named value slot on the stack. 

Pointer - NOTE - The name "reference", "reference pointer", "pointer reference" mean the same thing.
A value that holds the "address" of a region of memory so the pointer
points to a "place". To access the value held in the memory location of pointer,
you have to deference it. 

Variables - Named memory locations that may or may not hold legal values. A variable
is considered a "value" slot. When you assign to it, the slot is filled and if it had an old 
value, that would be dropped and replaced. When you try to access it, the compiler checks if 
it is empty and if so, it means either the value has NOT been initialized OR the value has
been moved. A pointer to a variable refers to the variable's "backing memory" and can be 
"derefenced" to get the value. 

Example - if we use "let x: usize", the variable "x" is a name for the memory region
on the Stack with room for a value of "usize" although it could be empty. If we assign 
a value to the variable "x = 6", that memory region will now contain the bits that represent
the value "6". A reference pointer such as "&x" does NOT change when you assign to "x". If you
declare multiple variables with the same name, there will be different chunks of memory
backing them. 

Stack - Used as a scratch space for function calls. When a function is called, a 
contiguous memory region called a "frame" is allocated at the top of the stack. Near
the bottom of the stack is the "main() function" This is because of the stack having a
"LIFO" methodology. Contained in the function frame are 

- All of the variables within that function
- Any arguments the function takes.

When the function returns, the stack frame is reclaimed and although the bytes that make up
the values are not immediately wiped they are inaccessible as they may have been overwritten
by a subsequent function call whose frame overlaps the reclaimed one. Also they may have illegal values
such as ones that were moved when the function returned.

Stack frames are very closely tied to "Lifetimes" in Rust. Any variable stored in the Stack that cannot be accessed
when the frame goes away MUST have a reference to it that is at most as long as the lifetime of the frame in question.

// TO DO - Add Heap and Static Memory Definitions

NOTE - In this example, we ignore CPU registers and treat them as an optimization. In reality,
the compiler may use a register to back a variable instead of a region of memory if
no memory address is needed for that variable.




**DROP ORDER**
Rust automatically drops values when they go out of scope, such as x1 and y1
in the inner scope in Listing 1-3. The rules for the order in which to drop are
fairly simple: variables (including function arguments) are dropped in reverse
order, and nested values are dropped in source-code order.
This might sound weird at first—why the discrepancy? If we look at it
closely, though, it makes a lot of sense. Say you write a function that declares
a string and then inserts a reference to that string into a new hash table. When
the function returns, the hash table must be dropped first; if the string were
dropped first, the hash table would then hold an invalid reference! In general,
later variables may contain references to earlier values, whereas the inverse
cannot happen due to Rust’s lifetime rules. And for that reason, Rust drops variables
in reverse order.
Now, we could have the same behavior for nested values, like the values
in a tuple, array, or struct, but that would likely surprise users. If you constructed
an array that contained two values, it’d seem odd if the last element of the array
were dropped first. The same applies to tuples and structs, where the most intuitive
behavior is for the first tuple element or field to be dropped first, then the
second, and so on. Unlike for variables, there is no need to reverse the drop
order in this case, since Rust doesn’t (currently) allow self-references in a single
value. So, Rust goes with the intuitive option.
