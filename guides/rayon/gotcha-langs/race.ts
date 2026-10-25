import { Worker, isMainThread, workerData, parentPort } from "node:worker_threads";

let total = 0; // each worker thread gets its own copy of this variable

if (isMainThread) {
  const workers = [];
  for (let p = 0; p < 8; p++) {
    const lo = p * 125_000 + 1;
    workers.push(new Promise<void>((done) => {
      const w = new Worker(new URL(import.meta.url), { workerData: { lo, hi: lo + 125_000 } });
      w.on("exit", () => done());
    }));
  }
  await Promise.all(workers);
  console.log("total:", total, "expected: 500000500000");
} else {
  for (let i = workerData.lo; i < workerData.hi; i++) total += i;
  parentPort!.postMessage("done");
}
