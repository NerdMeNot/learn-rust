import { Worker, isMainThread, workerData, parentPort } from "node:worker_threads";

// BigInt: the total is larger than 2**53, which a JavaScript number cannot hold exactly.
const N = 1_000_000n;
const PARTS = 8n;

if (isMainThread) {
  const sums = Array.from({ length: Number(PARTS) }, (_, p) => new Promise<bigint>((resolve) => {
    const part = BigInt(p);
    const lo = N * part / PARTS + 1n;
    const hi = N * (part + 1n) / PARTS;
    const worker = new Worker(new URL(import.meta.url), { workerData: { lo, hi } });
    worker.on("message", resolve);
  }));
  const total = (await Promise.all(sums)).reduce((a, b) => a + b, 0n);
  console.log(total.toString());
} else {
  let sum = 0n;
  for (let i = workerData.lo; i <= workerData.hi; i++) sum += i * i;
  parentPort!.postMessage(sum);
}
