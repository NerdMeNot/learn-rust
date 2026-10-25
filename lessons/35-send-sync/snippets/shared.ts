import { Worker, isMainThread, workerData } from "node:worker_threads";

if (isMainThread) {
  // memory that both sides can see; ordinary objects are copied instead
  const cell = new Int32Array(new SharedArrayBuffer(4));
  let running = 2;
  for (let i = 0; i < 2; i++) {
    new Worker(__filename, { workerData: cell }).on("exit", () => {
      if (--running === 0) console.log(`total: ${Atomics.load(cell, 0)}`);
    });
  }
} else {
  const cell: Int32Array = workerData;
  for (let i = 0; i < 1000; i++) Atomics.add(cell, 0, 1); // an atomic add, no lost updates
}
