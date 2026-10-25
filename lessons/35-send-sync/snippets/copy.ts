import { Worker, isMainThread, parentPort, workerData } from "node:worker_threads";

if (isMainThread) {
  const log: string[] = ["start"];
  const worker = new Worker(__filename, { workerData: log });
  worker.on("message", (n: number) => console.log(`worker's copy has ${n} entries`));
  worker.on("exit", () => console.log(`main's list has ${log.length} entry`));
} else {
  const log: string[] = workerData;
  log.push("from the worker"); // changes the worker's copy
  parentPort!.postMessage(log.length);
}
