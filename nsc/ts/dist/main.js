"use strict";
function main() {
    const n = 10;
    const fibN = fib(n);
    console.log(`fib(${n}) = ${fibN}`);
    const a = 84;
    const b = 30;
    const g = gcd(a, b);
    console.log(`gcd(${a}, ${b}) = ${g}`);
}
function fib(n) {
    if (n <= 1) {
        return n;
    }
    let prev = 0;
    let curr = 1;
    let i = 2;
    while (i <= n) {
        const next = prev + curr;
        prev = curr;
        curr = next;
        i = i + 1;
    }
    return curr;
}
function gcd(a, b) {
    let x = a;
    let y = b;
    while (y !== 0) {
        const r = x % y;
        x = y;
        y = r;
    }
    return x;
}
main();
