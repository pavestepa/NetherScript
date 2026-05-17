## Copyable Types
Copyable values can be duplicated without moving ownership. Passing a copyable value to a function does not make the original variable unusable.

Copyable types include:

* Integers: `i32`, `u32`, `isize`, `usize`, etc.
* Floating-point numbers: `f32`, `f64`, etc.
* Booleans: `boolean` (`true` and `false`).
* Characters: `char`.
* Shared references: `ref T`. References are copyable even when `T` is not.
* Function pointers: `(msg: String) => void`.
* Tuples: `(T, U)`, when all tuple elements are copyable.
* Enums: `enum Color { Red, Blue, Other(u32) }`, when all variants are copyable.

## Memory Management
Non-copyable values have two management models:

* `$Case` type names are ARC-managed heap objects. They are automatically retained and released.
* `PascalCase` type names are owned values. They use explicit ownership and borrowing.

### ARC-Managed Objects

```
let a = new $Animal();  // Allocates an ARC-managed object on the heap.
foo(a);                 // The function can use and mutate the object.
console.log(a);         // 'a' is still usable because the object is retained.

putAnimalInZoo(a);      // Passes another reference to the same object.
```

## Borrow Checking
For performance-sensitive code, use owned values with explicit ownership and borrowing, similar to Rust.

```
let a = new Animal();   // Allocates an owned Animal value. The type is 'own Animal'.
                        // To allocate it on the heap, use: own Box(Animal()).
foo(&mut a);            // Borrows 'a' as a mutable reference.
console.log(&a);        // Borrows 'a' as a read-only reference for logging.

putAnimalInZoo(a);      // Moves ownership of the Animal value into the function.
                        // 'a' cannot be used after this point.
```