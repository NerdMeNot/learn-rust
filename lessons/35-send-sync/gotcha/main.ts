import { Worker, isMainThread, parentPort, workerData } from "node:worker_threads";

if (isMainThread) {
  const log: string[] = [];
  const job = { name: "resize", log };
  // the worker receives a copy of `job`, not the same object
  const worker = new Worker(__filename, { workerData: job });
  worker.on("exit", () => {
    console.log(`log: [${log.map((s) => JSON.stringify(s)).join(", ")}]`);
  });
} else {
  workerData.log.push(workerData.name); // changes the worker's copy only
  parentPort!.postMessage("done");
}
