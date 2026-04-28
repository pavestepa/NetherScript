function main(): void {
  console.time("main");

  const n: number = 31;
  const fibN: number = fib(n);
  console.log(`fib(${n}) = ${fibN}`);

  let i = 0;
  while (i < fibN) {
    console.log(`${i}`);
    i = i + 1;
  }

  console.timeEnd("main");
}

function fib(n: number): number {
  if (n <= 1) {
    return n;
  }

  let prev: number = 0;
  let curr: number = 1;
  let i: number = 2;

  while (i <= n) {
    const next: number = prev + curr;
    prev = curr;
    curr = next;
    i = i + 1;
  }

  return curr;
}

main();
