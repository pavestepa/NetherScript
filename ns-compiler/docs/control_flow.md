# Control Flow
This file describes branching, loops, pattern matching, and early exits.

## If Expressions
`if` can be used as a normal statement. `if` can also be used as an expression that returns a value.

```ns
if (num == 2) {
  console.log("num is 2");
} else if (num > 2) {
  console.log("num is greater than 2");
} else {
  console.log("num is less than 2");
}
```

Possible expression form:

```ns
const label = if (num == 2) {
  "two"
} else {
  "other"
};
```

## Loops
NetherScript should support common TypeScript-like loops.

```ns
while (isRunning) {
  tick();

  if (shouldStop()) {
    break;
  }
}

for (const item of items) {
  if (item.isHidden) {
    continue;
  }

  render(item);
}

for (let i = 0; i < 10; i += 1) {
  console.log(i);
}
```

## Pattern Matching
TODO: describe matching on enums, tuples, literals, ranges, and destructuring.

```ns
match num {
  1 -> console.log("num is 1"),
  2 -> console.log("num is 2"),
  other -> console.log(`num is ${other}`),
}
```
```ns
function isHomeAnimal(someAnimal: Animal) {
  match someAnimal {
    Animal.Cat -> {
      console.log("Is home animal!");
      return true;
    }
    Animal.Dog -> {
      console.log("Is home animal!");
      return true;
    }
    e -> {
      console.log("not sure about this animal");
      return false;
    }
  }
}
```

## Early Returns
Functions can exit early with `return`. Recoverable errors can be propagated with `throw` or another error mechanism. TODO: decide final error propagation syntax.

```ns
function findUser(id: u32): Option<User> {
  if (id == 0) {
    return None;
  }

  const user = users.get(id);

  if (user == null) {
    return None;
  }

  return user;
}

function requireUser(id: u32): User {
  const user = findUser(id);

  if (user == null) {
    throw new Error("user not found");
  }

  return user;
}
```

## Implementation Notes
Proposed implementation model:

* Parse `if`, loops, `match`, `break`, `continue`, `return`, and `throw` as control-flow nodes in the AST.
* Lower each function body into basic blocks before type checking ownership-sensitive flow.
* Check that all branches return a compatible type when `if` or `match` is used as an expression.
* Track reachability after `return`, `throw`, `break`, and `continue`.
* For `match`, verify that enum matches are exhaustive or require a fallback binding like `other`.
* For loops, track whether `break` and `continue` target a valid enclosing loop.