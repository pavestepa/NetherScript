# NetherScript

A concise overview of the **NetherScript** language syntax and concepts.

---

## Declarations

```text
function foo(): i32 { ... }                     // default function declaration

class Car { ... }                               // default heap-allocated class with `extend` 
                                                //inheritance

                                                // and trait implementations. All fields are public by default.
                                                // Prefix an identifier with '_' to make it private.

trait Buyable { ... }                           // traits (similar to Rust)

struct Animal { ... }                           // data-only structure. All fields are public by 
                                                // default.

                                                // Prefix an identifier with '_' to make it       
                                                //private.

enum Color { Red, White, Blue, Other(String) }  // enum like in Rust, with associated data

implement Animal { ... }                        // `impl`-like block (methods only).
                                                // Prefix an identifier with '_' to make it 
                                                // private.
                                                // Trait implementations are always public.

implement Speaking for Animal { ... }           // `implement` can be used for all data types:
                                                // structs, enums, primitives (e.g., i32), String, 
                                                // etc.
                                                // Not allowed for classes.

const SOME = 4;                                 // global compile-time constant
```

> See more declarations in **Module System**.

---

## Statements

```text
{ ... }                      // block of code to encapsulate scope across multiple lines

return                       // return a value from a function

break                        // break out of a loop

var a: i32                   // initialize a variable with `var` (mutable) or `let` (immutable)
let b: u32

foo(), a, 4, a = 34, a > b.x // expressions

if a > b { ... }             // classic if / else

while a > 0 { ... }          // classic while loop

match a { ... }              // pattern matching (like Rust)
```

---

## Module System

### Declarations

```text
// NetherScript             // Rust

index cat;                  mod cat;
index dog;                  mod dog;

import Cat from cat;        pub use cat::Cat;
import Dog from dog;        pub use dog::Dog;

export {
    Cat,
    Dog,
}
```

### Notes

* `index` — similar to `mod` in Rust.
* `import` — similar to `use` in Rust.
* `export` — adds the `pub` modifier to `import` and `index` declarations.

---

## Conventions

* Identifiers prefixed with `_` are **private**.
* Everything else is **public by default** unless stated otherwise.
* The language borrows familiar concepts from Rust while remaining flexible and expressive.
