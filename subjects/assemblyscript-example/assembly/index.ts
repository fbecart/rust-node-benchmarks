// The entry file of your WebAssembly module.

export function sum(a: i32, b: i32): i32 {
  return a + b;
}

export function fibonacci(n: i32): i32 {
  let a = 1, b = 1;
  for (let i = 0; i < n; i++) {
    let sum = a + b;
    a = b;
    b = sum;
  }
  return b;
}
