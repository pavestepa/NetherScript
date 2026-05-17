# Errors
This file describes recoverable errors, panics, diagnostics, and error propagation.

NetherScript error handling should be familiar to TypeScript users. Recoverable failures use `throw`, `try`, `catch`, and `finally`. Async functions reject their returned `Promise<T>` when they throw.

## Recoverable Errors
The primary user-facing model is exception-like errors. Any value that implements or extends `Error` can be thrown.

```ns
class FileNotFoundError extends Error {
  constructor(path: string) {
    super(`file not found: ${path}`);
  }
}

function readConfig(path: string): Config {
  if (!fs.exists(path)) {
    throw new FileNotFoundError(path);
  }

  return parseConfig(fs.readText(path));
}
```

TODO: decide whether throwing arbitrary values is allowed or only `Error` values are throwable.

## Error Propagation
Errors propagate until they are caught by a matching `catch` block. `finally` always runs before control leaves the `try` block.

```ns
function startApp() {
  try {
    const config = readConfig("./app.ns.json");
    run(config);
  } catch (error) {
    console.error(error.message);
  } finally {
    console.log("startup attempt finished");
  }
}
```

Typed catch blocks may be supported for narrowing.

```ns
try {
  readConfig("./app.ns.json");
} catch (error: FileNotFoundError) {
  console.error(`missing file: ${error.path}`);
} catch (error) {
  console.error(error.message);
}
```

TODO: decide whether typed `catch` is syntax-level matching or normal runtime `instanceof` checking.

## Async Errors
Throwing inside an async function rejects the returned promise. Awaiting a rejected promise throws at the await site.

```ns
async function loadUser(id: u32): Promise<User> {
  const response = await fetch(`/users/${id}`);

  if (response.status == 404) {
    throw new Error("user not found");
  }

  return await response.json();
}

async function main(): Promise<void> {
  try {
    const user = await loadUser(1);
    console.log(user.name);
  } catch (error) {
    console.error(error.message);
  }
}
```

## Panics
`panic` is for unrecoverable bugs and violated invariants. It should abort the current task or program instead of being treated as normal application control flow.

```ns
function getUnchecked<T>(items: Array<T>, index: usize): T {
  if (index >= items.length) {
    panic(`index out of bounds: ${index}`);
  }

  return items[index];
}
```

Expected behavior:

* `panic(message)` records the message and source location.
* In debug builds, panic output should include a stack trace when possible.
* In async code, panic should fail the current task. TODO: decide whether it aborts the whole program or only rejects/terminates the current task.
* Panics are not intended to be caught by `catch`.

## Compiler Diagnostics
A good compiler diagnostic should include an error code, source span, primary message, optional notes, and optional fix suggestions.

```text
error[E0301]: cannot use moved value `user`
  --> src/main.ns:12:15
   |
 9 |   saveUser(user);
   |            ---- value moved here
10 |
12 |   console.log(user.name);
   |               ^^^^ value used after move
   |
note: `User` is not copyable
help: pass a reference instead: `saveUser(&user)`
```

## Implementation Notes
Proposed implementation model:

* `Error` is a standard library base type with at least `name`, `message`, and optional `cause`.
* `throw expr` lowers to runtime error propagation.
* `try/catch/finally` lowers to explicit control-flow edges in the compiler IR.
* Async `throw` lowers to promise rejection.
* Awaiting a rejected promise resumes the caller by throwing the rejection reason.
* `panic` lowers to a runtime intrinsic that records message, source span, and stack trace if available.
* Compiler diagnostics should be structured data first, formatted text second.
