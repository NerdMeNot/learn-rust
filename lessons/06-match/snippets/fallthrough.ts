function f(n: number): string {
  let s = "";
  switch (n) {
    case 1:
      s = "one";
    case 2:
      s += "two";
      break;
  }
  return s;
}

console.log(f(1));
