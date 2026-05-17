## Non-Copyable Data Initialization
NetherScript uses naming conventions to show how non-copyable data is managed:

* `$Case` types are ARC-managed heap objects.
* `PascalCase` types are owned values that participate in borrow checking.

The examples below show the ARC-managed form on the left and the owned form on the right.

## Constructors

```text
$Animal                     | Animal
ARC-managed heap object     | Owned value
----------------------------|----------------------------
new $Animal();              | new Animal()
```

## String Literals

```text
string                      | String
Copyable string value       | Owned string value
----------------------------|----------------------------
$"my type name is string"   | "my type name is String"
$`my type name is {type}`   | `my type name is {type}`
```

## Object Literals

```text
$User                       | User
ARC-managed object          | Owned value
----------------------------|----------------------------
$User { name: 'Paul' }      | User { name: "Paul" }
```

## Array Literals

```text
u32[] / $Array<u32>         | Array<u32>
ARC-managed array           | Owned array
----------------------------|----------------------------
$[1, 2, 3, 4]               | @Array[1, 2, 3, 4]
new $Array()                | new Array()
new $Array(1, 2)            | new Array(1, 2) // or new Vec([1, 2])
```

## Stack-Allocated Static Arrays

Static arrays have a fixed size known at compile time and are owned values.

```text
[u32; 4]
----------------------------
[2, 3, 5, 6]
```