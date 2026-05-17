# Async
This file describes async functions, promises, await, cancellation, multithreading, and runtime integration.

NetherScript async APIs should feel close to TypeScript. User code works with `async`, `await`, and `Promise<T>`. Tokio is embedded internally as the runtime implementation detail, so users do not create executors or depend on Tokio APIs directly.

## Async Functions
Async functions return `Promise<T>`. The body can use `await`, and the returned value is automatically wrapped into a resolved promise.

```ns
async function fetchUser(id: u32): Promise<User> {
    const response = await fetch(`/users/${id}`);
    return await response.json();
}
```

## Await
`await` waits for a promise and returns its resolved value. It is valid inside async functions and async closures.

```ns
const user = await fetchUser(1);
console.log(user.name);
```

TODO: decide whether top-level `await` is allowed in entry modules.

```ns
// TODO: top-level await example, if supported.
```

## Promises
The public async type is `Promise<T>`, not `Future<T>`. Internally, promises can be lowered to Rust futures, but this should not be visible in NetherScript source code.

```ns
const promise: Promise<User> = fetchUser(1);
const user = await promise;
```

## Promise API
The standard library should provide a TypeScript-like `Promise` API.

```ns
const user = await Promise.resolve(existingUser);
const failed = Promise.reject(new Error("failed"));

const results = await Promise.all([
    fetchUser(1),
    fetchUser(2),
]);

const fastest = await Promise.race([
    requestFromPrimary(),
    requestFromReplica(),
]);
```

TODO: decide exact support for `Promise.allSettled`, `Promise.any`, `finally`, and chaining with `.then()` / `.catch()`.

## Cancellation
Cancellation should be close to the TypeScript `AbortController` model.

```ns
const controller = new AbortController();

const promise = fetch("/users/1", {
    signal: controller.signal,
});

controller.abort();
await promise;
```

TODO: define how cancellation maps to dropped Rust futures and which standard APIs must be cancellation-safe.

## Timers
Timers should use browser/TypeScript-like names while lowering to Tokio timers internally.

```ns
setTimeout(() => {
    console.log("later");
}, 1000);

const value = await sleep(1000);
```

TODO: decide whether `setInterval`, `clearTimeout`, and `clearInterval` are part of the core standard library.

## Multithreading
Normal async code should not require special task or executor syntax. Separate syntax is only needed when work must run on another OS thread or blocking thread pool.

Proposed multithreading API:

```ns
const handle = thread.spawn(() => {
    return heavyCpuWork();
});

const result = await handle.join();
```

Blocking work can be moved away from the async runtime:

```ns
const result = await thread.spawnBlocking(() => {
    return readLargeFileSync(path);
});
```

TODO: decide ownership rules for values moved into threads. Thread closures probably require owned, sendable values.

## Standard Runtime
Tokio is embedded as the standard NetherScript async runtime.

Expected behavior:

* Async entry points are wrapped in a Tokio runtime automatically.
* `Promise<T>` is the public source-level async type.
* `async function` lowers to Rust futures managed by the runtime.
* `await` lowers to `.await`.
* Timers lower to `tokio::time`.
* Async IO lowers to Tokio-backed IO types.
* `thread.spawn(...)` lowers to Tokio task spawning or native threads, depending on the chosen implementation.
* `thread.spawnBlocking(...)` lowers to `tokio::task::spawn_blocking(...)`.
* User code should not depend on Tokio names, Rust futures, or Rust-specific runtime types.

```ns
async function main(): Promise<void> {
    const user = await fetchUser(1);
    console.log(user.name);

    const result = await thread.spawnBlocking(() => {
        return heavyCpuWork();
    }).join();
}
```

## Implementation Notes
Proposed implementation model:

* The parser accepts `async function`, async closures, `await`, `Promise<T>`, and promise combinators.
* The type checker treats async functions as returning `Promise<T>`, where `T` is the resolved value type.
* The borrow checker must reject references that cannot safely live across suspension points.
* The lowering step converts async bodies into Rust async functions, async blocks, or explicit state machines.
* The runtime crate owns Tokio setup and exposes a stable NetherScript promise ABI.
* `Promise<T>` can be represented internally as a pinned boxed Rust future, a runtime-managed task, or another wrapper chosen by the compiler.
* The generated program entry point creates the Tokio runtime and runs the NetherScript entry promise to completion.
* Standard library async APIs are thin wrappers around Tokio, keeping Tokio as an implementation detail.
* Multithreaded closures must be checked for sendability and ownership before they are lowered to Tokio tasks or native threads.


