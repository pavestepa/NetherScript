```Rust
fn add_num(nums: &mut Vec<i32>) {
    nums.push(8);
}

fn main() {
    let mut nums = vec![1, 2, 4];

    add_num(&mut nums);

    println!("{:?}", nums);
}

```
``` TypeScript // only for coloring syntax
function addNum(nums: change Vec<i32>) {
    nums.push(8);
}

function main() {
    let nums = vec![1, 2, 4];

    add_num(change nums);

    println!("{:?}", read nums);
}
```
