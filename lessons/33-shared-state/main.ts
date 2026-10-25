import { Worker, isMainThread, workerData } from "node:worker_threads";
import { once } from "node:events";

// shared memory layout: [lock, length, item0, item1, item2]
const LOCK = 0;
const LEN = 1;

function lock(m: Int32Array): void {
  while (Atomics.compareExchange(m, LOCK, 0, 1) !== 0) {
    Atomics.wait(m, LOCK, 1); // sleep until the holder unlocks
  }
}

function unlock(m: Int32Array): void {
  Atomics.store(m, LOCK, 0);
  Atomics.notify(m, LOCK, 1);
}

async function main(): Promise<void> {
  // SharedArrayBuffer is the one thing workers really share
  const shared = new Int32Array(new SharedArrayBuffer(4 * 5));
  const workers = [0, 1, 2].map((i) => new Worker(__filename, { workerData: { shared, i } }));
  await Promise.all(workers.map((w) => once(w, "exit")));

  const results = Array.from(shared.subarray(2, 2 + shared[LEN])).sort((a, b) => a - b);
  console.log(results);
}

if (isMainThread) {
  main();
} else {
  const { shared, i } = workerData as { shared: Int32Array; i: number };
  lock(shared);                       // lock, push, unlock
  shared[2 + shared[LEN]] = i * 10;
  shared[LEN] += 1;
  unlock(shared);
}
