import { Worker, isMainThread, parentPort, workerData } from "node:worker_threads";

if (isMainThread) {
  const names = ["a", "b", "c"];
  // this same file runs again in the worker; names is copied to it
  const worker = new Worker(__filename, { workerData: names });

  let count = 0;
  worker.on("message", (n: number) => (count = n)); // the worker's answer
  worker.on("exit", () => console.log(`worker counted ${count}`));
} else {
  const names: string[] = workerData;
  for (const n of names) {
    console.log(`worker sees ${n}`);
  }
  parentPort!.postMessage(names.length);
}
