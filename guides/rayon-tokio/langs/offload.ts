import { Worker, isMainThread, parentPort } from "node:worker_threads";

function work(): number {
  let total = 0;
  for (let n = 0; n < 1_000_000; n++) total += n % 7;
  return total;
}

async function main() {
  if (isMainThread) {
    const total = await new Promise<number>((resolve, reject) => {
      const worker = new Worker(__filename); // the same file, started on another thread
      worker.once("message", resolve);
      worker.once("error", reject);
    });
    console.log("worker thread:", total);
  } else {
    parentPort!.postMessage(work());
  }
}

main();
