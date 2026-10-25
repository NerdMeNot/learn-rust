import { Worker } from "node:worker_threads";

// the worker waits for a message, so it cannot print before main has
const worker = new Worker(
  `require("node:worker_threads").parentPort.once("message", () =>
     console.log("worker: finished after main reached the end of the file"));`,
  { eval: true },
);
worker.on("exit", (code: number) => console.log(`main: worker exited with code ${code}`));
console.log("main: reached the end of the file");
worker.postMessage("go");
