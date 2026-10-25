import { Worker, isMainThread, workerData } from "node:worker_threads";
import { once } from "node:events";

async function main(): Promise<void> {
  const counter = new Int32Array(new SharedArrayBuffer(4)); // memory both sides can see
  const workers = [0, 1, 2, 3].map(() => new Worker(__filename, { workerData: counter }));
  await Promise.all(workers.map((w) => once(w, "exit")));
  console.log("Atomics.add from 4 workers:", Atomics.load(counter, 0));
}

if (isMainThread) {
  main();
} else {
  const counter = workerData as Int32Array;
  for (let i = 0; i < 1000; i++) {
    Atomics.add(counter, 0, 1);
  }
}
