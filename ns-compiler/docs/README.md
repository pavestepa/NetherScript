# NetherScript Language Notes
This folder describes how NetherScript should behave and leaves space for implementation notes.

Suggested reading order:

* `syntax.md`: source file structure, declarations, expressions, and literals.
* `types.md`: copyable types, ownership, ARC-managed values, and borrowing.
* `user_defined_types.md`: structs, objects, enums, traits, and interfaces.
* `data_init.md`: initialization forms for non-copyable data.
* `functions.md`: function declarations, calls, closures, and generics.
* `operators.md`: operators, precedence, assignment, access, and overload rules.
* `control_flow.md`: conditionals, loops, pattern matching, and early returns.
* `modules.md`: files, modules, imports, visibility, and packages.
* `errors.md`: recoverable errors, panics, diagnostics, and result types.
* `async.md`: async functions, tasks, futures, await, and runtime behavior.
* `stdlib.md`: standard library layout and required core APIs.

Each file contains TODO blocks for:

* Language syntax examples.
* Semantics and edge cases.
* Parser, type checker, and runtime/compiler implementation notes.
