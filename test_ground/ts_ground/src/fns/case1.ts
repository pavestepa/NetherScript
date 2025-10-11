export function case1() {
  const a = { num: 1 };
  const b = a;
  let c = a;

  a.num = 2;
  b.num = 4;
  c.num = 5;

  console.log(a);
  console.log(b);
  console.log(c);

  c = { num: 10 };

  console.log(c);
}
