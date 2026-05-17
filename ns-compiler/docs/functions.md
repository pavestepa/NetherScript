# Functions
This file describes function declarations, calls, closures, and related type rules.

Function syntax should stay close to TypeScript, but argument passing follows NetherScript ownership rules.

## Function Declarations
Functions use `function name(args): ReturnType`. If the return type is omitted, TODO: decide whether it is inferred or treated as `void`.

Parameter behavior depends on the type:

* Copyable values like `i32`, `boolean`, `char`, and `ref T` are copied.
* `$Case` ARC-managed values are passed by retained reference.
* `PascalCase` owned values are moved when passed by value.
* `&T` borrows a value as read-only.
* `&mut T` borrows a value as mutable.

```ns
function add(left: i32, right: i32): i32 {
  return left + right;
}

function printUser(user: $User): void {
  console.log(user.name);
}

function renameUser(user: &mut User, name: String): void {
  user.name = name;
}

function putAnimalInZoo(animal: Animal): void {
  // Takes ownership of `animal`.
}

function shareAnimal(animal: $Animal): void {
  // Receives another retained reference to the same ARC-managed object.
}
```

## Calls
Function calls are positional by default. TODO: decide whether named arguments are supported.

```ns
const sum = add(1, 2);

let arcAnimal = new $Animal();
shareAnimal(arcAnimal);
console.log(arcAnimal.name); // still usable

let ownedAnimal = new Animal();
feedAnimal(&mut ownedAnimal);
inspectAnimal(&ownedAnimal);
putAnimalInZoo(ownedAnimal);
// ownedAnimal cannot be used after move
```

## Function Overloads
Functions can be overloaded by parameter types and parameter passing modes. Return type alone is not enough to create a separate overload.

Overload resolution must consider ownership and borrow modes, not only type names:

* Exact type matches are preferred.
* Explicit borrows like `&user` and `&mut user` select borrowed overloads.
* Passing an owned `PascalCase` value by value selects a consuming overload and moves the value.
* Passing a `$Case` value by value selects an ARC-managed overload and retains the object.
* Copyable values can match by-value parameters without moving ownership.
* If two overloads are equally good, the call is ambiguous and must be rejected.

```ns
function save(user: $User): void {
  // Save shared ARC-managed user.
}

function save(user: &User): void {
  // Save borrowed owned user.
}

function save(user: User): void {
  // Save owned user and consume it.
}

let shared = new $User();
save(shared); // selects save($User)

let owned = new User();
save(&owned); // selects save(&User), owned is still usable
save(owned);  // selects save(User), owned is moved
```

Overloads can also be generic, but non-generic exact matches should be preferred over generic matches.

```ns
function print(value: string): void {
  console.log(value);
}

function print<T: Display>(value: &T): void {
  console.log(value.toString());
}

print($"hello"); // selects print(string)
```

## Function Types
Function types use arrow syntax. Parameter types include ownership and borrow markers.

```ns
let logger: (msg: string) => void = (msg) => {
  console.log(msg);
};

let visitor: (animal: &Animal) => boolean = (animal) => {
  return animal.isHomeAnimal;
};

let consumer: (animal: Animal) => void = (animal) => {
  putAnimalInZoo(animal);
};
```

## Closures
Closures should infer parameter and return types when possible. Captures follow the same ownership rules as function calls.

Default capture behavior:

* Copyable captured values are copied.
* `$Case` captured values are retained.
* Owned `PascalCase` values are borrowed if the closure does not outlive the current scope.
* `move` closures take ownership of captured owned values.
* Async and threaded closures may require `move` because they can outlive the current stack frame.

```ns
const prefix = $"user:";
const formatName = (name: string): string => {
  return `${prefix} ${name}`;
};

let animal = new Animal();

const inspect = () => {
  console.log((&animal).name);
};

const consume = move () => {
  putAnimalInZoo(animal);
};
```

Async closures should match the TypeScript-like async model from `async.md`.

```ns
const load = async (id: u32): Promise<$User> => {
  return await fetchUser(id);
};
```

## Generics
Generic functions use TypeScript-like type parameters. Constraints should describe required operations, traits, interfaces, or ownership properties.

```ns
function identity<T>(value: T): T {
  return value;
}

function cloneCopyable<T: Copy>(value: T): T {
  return value;
}

function first<T>(items: &Array<T>): &T {
  return &items[0];
}

function map<T, U>(items: &Array<T>, f: (item: &T) => U): Array<U> {
  let result = new Array<U>();

  for (const item of items) {
    result.push(f(&item));
  }

  return result;
}
```

TODO: decide final constraint names: `Copy`, `Send`, `Sync`, `Clone`, `Drop`, or NetherScript-specific equivalents.

## Methods
Methods are functions attached to a type. Receiver type controls ownership:

* `&this` for read-only access.
* `&mut this` for mutation.
* `this` for consuming methods.
* `this` in `$Case` methods receive retained ARC-managed objects by default unless explicitly borrowed.

```ns
User extends {
  displayName(&this): String {
    return this.name;
  }

  rename(&mut this, name: String) {
    this.name = name;
  }

  intoArchive(this): ArchivedUser {
    return ArchivedUser {
      name: this.name,
    };
  }
}

$User extends {
  displayName(this): string {
    return this.name;
  }
}
```

## Implementation Notes
Proposed implementation model:

* Parse functions into declarations with parameter list, optional type parameters, return type, and body.
* Resolve function names into overload sets before type checking bodies, so functions can call declarations that appear later in the file.
* Type checking a call first selects the best overload, then computes the passing mode for each argument: copy, ARC retain, move, shared borrow, or mutable borrow.
* Overload candidates are ranked by exactness, required coercions, borrow mode, generic inference, and whether the call would move an owned value.
* Ambiguous overload calls are compiler errors.
* Passing a non-copyable owned value by value marks the source binding as moved.
* Passing `$Case` by value emits an ARC retain/release pair around the call as needed.
* Passing `&T` or `&mut T` creates a borrow region checked by the borrow checker.
* Closures are lowered to generated structs containing captured values or references.
* `move` closures store owned captures by value.
* Async closures lower to promise-returning state machines.
* Generic functions can be monomorphized first. TODO: decide whether dynamic dispatch is also supported for constrained generics.