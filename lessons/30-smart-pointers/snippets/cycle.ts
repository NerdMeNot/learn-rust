import * as v8 from "node:v8";
import * as vm from "node:vm";

// node only exposes gc() when asked; this asks from inside the program
v8.setFlagsFromString("--expose-gc");
const gc: () => void = vm.runInNewContext("gc");

class Node {
  other: Node | null = null;
}

function makeCycle(): WeakRef<Node> {
  const a = new Node();
  const b = new Node();
  a.other = b;
  b.other = a;
  return new WeakRef(a); // does not keep a alive
}

async function main() {
  const weak = makeCycle();
  // a WeakRef's target is kept alive until the current job ends
  await new Promise((resolve) => setTimeout(resolve, 0));
  gc();
  console.log("cycle collected:", weak.deref() === undefined);
}

main();
