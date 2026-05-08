## Copyable Types
Copyable values can be duplicated without transferring ownership.

* Integers: *i32*, *u32*, *isize*, *usize*, etc.
* Floating-point numbers: *f32*, *f64*, etc.
* Booleans: *boolean* (*true* and *false*).
* Characters: *char*.
* Shared references: *ref T*. They are always copyable, even when *T* is not.
* Function pointers: *(msg: String) => void*.
* Tuples: *(T, U)*, when all tuple elements are copyable.

## Memory Management
Non-copyable values are managed by ownership. Heap-allocated objects created with *new* are automatically retained and released by ARC.

```
    let a = new Animal();   // Allocate an Animal object on the heap. The type is 'Animal'.
    foo(a);                 // The function can use and mutate the object.
    console.log(a);

    putAnimalInZoo(a);      // Transfer the value to another function.
```

This lets ordinary code feel similar to TypeScript or other GC languages, while the compiler still inserts memory-management operations automatically.

## Borrow Checking
For performance-sensitive code, values can be managed through explicit ownership and borrowing, similar to Rust.

```
    let a = own Animal();   // Allocate an owned Animal value on the stack. The type is 'own Animal'.
                            // To allocate it on the heap, use: own Box(Animal()).
    foo(mut a);             // Borrow 'a' as a mutable reference.
    console.log(ref a);     // Borrow 'a' as a read-only reference for logging.

    putAnimalInZoo(a);      // Move ownership of the Animal value into the function.
```