function add(a: number, b: number): number {
  return a + b;
}

function greet(name: string): void { // void: returns nothing
  console.log(`hi ${name}`);
}

console.log(add(2, 3));
greet("kotlin");
