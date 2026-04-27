# ns-sema checks and typing model

This document describes what semantic checks are performed by `compiler/ns-sema` and which
typing rules the compiler relies on before Rust code generation.

## Pipeline

Semantic analysis runs in this order:

1. `collect_decls` - collect top-level declarations into symbol tables.
2. `resolve_names` - resolve identifiers in type/value namespaces.
3. `check_types` - infer/check expression and statement types.

Builtins and prelude are seeded before these passes.

## Namespaces

`ns-sema` uses separate lexical namespaces:

- `type_names` - classes, interfaces, enums, type aliases, type parameters, builtins.
- `value_names` - functions, constants, locals, parameters, prelude values.

This allows `type Foo = ...` and `const Foo = ...` to coexist safely.

## Builtin and prelude symbols

Global builtins:

- `boolean`, `String`, `void`, `never`
- Rust-like numeric/scalar primitives:
  - signed ints: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
  - unsigned ints: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
  - floats: `f32`, `f64`
  - scalars/text: `char`, `str`

These are interned from a single builtin catalog (`builtins::BUILTIN_TYPE_CATALOG`) into stable
builtin handles (`ctx.builtins`) and declared in type namespace.

Virtual prelude (`std::prelude`) is seeded implicitly (unless disabled via analysis options):

- Types: `Any`, `Number`, `ToString`, `Debug`
- Values: `print`, `panic`

User declarations shadow prelude symbols as normal lexical shadowing.

## Type checks currently performed

`check_types` currently validates:

1. **Type node lowering**
   - `TypeNode` is lowered into internal `TypeId` entries.
   - Named and dynamic types are interned.

2. **Literal typing**
   - numeric literal -> `i32`
   - string literal -> `String`
   - boolean literal -> `boolean`

3. **Binding and assignment checks**
   - binding initializer must be assignable to annotated type (or inferred from RHS).
   - assignment RHS must be assignable to LHS variable type.
   - local `let` variable must be definitely initialized before read.
   - after `if`, variable is considered initialized only if initialized in both branches.
   - for `while`/`loop` in MVP mode, initialization inside the loop is not propagated outside.

4. **Control flow checks**
   - `if`/`while` conditions must be `boolean`.
   - `return` value must match function/method return type.

5. **Operator checks**
   - arithmetic operators require `i32` operands.
   - logical comparisons require compatible operand types.

6. **Call checks**
   - function call argument count is validated.
   - each argument type must be assignable to declared parameter type.
   - special prelude handling: `print` -> `void`, `panic` -> `never`.

7. **Dynamic/interface checks**
   - `dynamic Interface` must reference an interface declaration.

## Inheritance model (Java-like in source, Go-style embedding in Rust lowering)

NetherScript classes use Java-like nominal inheritance (`extends`).

Assignability rule:

- If `Dog extends Animal`, then `Dog` is assignable to `Animal`.
- This is transitive: if `C extends B` and `B extends A`, then `C` is assignable to `A`.

`check_types` enforces this via subclass checks during assignment/call argument/return compatibility.

### Rust codegen expectation

For Rust backend, this inheritance is expected to lower as explicit embedding:

- NS source: `Dog extends Animal`
- Rust shape:
  - `struct Dog { inherited_class: Animal, ...dog_fields }`

When a function expects `Animal`, passing `Dog` is lowered by selecting embedded base:

- NS: `getAnimal(dog)`
- Rust lowering idea: `get_animal(dog.inherited_class)`

This keeps source-level inheritance semantics while generating Rust-compatible data layout and calls.

## Diagnostics

`ns-sema` diagnostics are structured and rendered in a rustc-like text format:

- severity (`error`/`warning`)
- optional diagnostic code (`E06xx`)
- message
- semantic snippet (AST-focused excerpt)
- notes (`= note: ...`)

## Error Code Table

| Code | Category | Meaning |
|---|---|---|
| `E0600` | Dynamic/interface | `dynamic` type references a symbol that is not an interface. |
| `E0601` | Assignment/member | Member assignment typing is not implemented because `MemberExpr` internals are not exposed. |
| `E0602` | Control flow | `if` condition type is not `boolean`. |
| `E0603` | Control flow | `while` condition type is not `boolean`. |
| `E0604` | Name/type env | Value was not found in checker value environment (local + global). |
| `E0605` | Unary operators | Numeric unary operator (`+` / `-`) got a non-`i32` operand. |
| `E0606` | Unary operators | Logical unary operator (`!` / bit-not mode in current checker) got a non-`boolean` operand. |
| `E0607` | Binary operators | Arithmetic binary operator got non-`i32` operand(s). |
| `E0608` | Binary operators | Logical comparison operands are not mutually assignable. |
| `E0609` | Calls | Function call argument count mismatch. |
| `E0610` | Calls | Non-named call target is not type-checked yet (no first-class callable typing). |
| `E0611` | Member access | Member expression typing is blocked by current AST API limitations. |
| `E0612` | Dynamic/interface | `dynamic` references an unknown interface. |
| `E0613` | Assignability | Generic expected/found type mismatch (binding, return, call arg, etc.). |
| `E0614` | Calls | Unknown callable: symbol is neither declared function nor prelude callable (`print`/`panic`). |
| `E0615` | Classes/interfaces | `class ... implements ...` references a non-interface symbol. |
| `E0616` | Classes/inheritance | `class ... extends ...` references an unknown/non-class base. |
| `E0617` | Classes/constructors | `new` targets an unknown class declaration. |
| `E0618` | Classes/layout | Duplicate field name inside one class body. |
| `E0619` | Classes/layout | Duplicate method name inside one class body. |
| `E0620` | Classes/inheritance | Cyclic inheritance detected in class extends chain. |
| `E0621` | Interfaces/contracts | Class is missing interface-required method. |
| `E0622` | Interfaces/contracts | Method arity does not match interface method contract. |
| `E0623` | Interfaces/contracts | Method argument type does not match interface method contract. |
| `E0624` | Interfaces/contracts | Method return type does not match interface method contract. |
| `E0625` | Interfaces/contracts | Method receiver kind (`this` vs static) does not match interface contract. |
| `E0626` | Interfaces/layout | Duplicate method declaration in one interface body. |
| `E0627` | Type modifiers | `extends` / `implements` modifier target must be a named type. |
| `E0628` | Type modifiers/layout | Duplicate method name inside one type-modifier block. |
| `E0629` | Type modifiers/layout | Method name in modifier duplicates an existing method of target type (or another modifier). |
| `E0630` | Enums/layout | Duplicate enum member/variant name in one enum body. |
| `E0631` | Enums/interfaces | `enum ... implements ...` references a non-interface symbol. |
| `E0632` | Enums/contracts | Enum is missing interface-required method. |
| `E0633` | Enums/layout | Duplicate method declaration in one enum body. |
| `E0634` | Types/env | Type name is missing from checker global type environment. |
| `E0635` | Members | Unknown member access or member access on non-object type. |
| `E0636` | Members | Computed member access (`obj[key]`) typing is not implemented yet. |
| `E0637` | Definite assignment | Use of possibly uninitialized local variable. |

Example shape:

```text
error[E0613]: assignment type mismatch
  --> <semantic>
   |
   | binding x
   |
   = note: expected `Animal`
   = note: found `Dog`
```

Note: precise file/line spans are not available yet in `ns-ast`, so diagnostics currently print semantic snippets.
When source spans are added to AST nodes, diagnostics should be upgraded to print exact source excerpts.
