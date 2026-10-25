import { Worker } from "node:worker_threads";

const names = ["a", "b", "c"];
const counter = new Int32Array(new SharedArrayBuffer(4));
const worker = new Worker(
  `const { workerData, parentPort } = require("node:worker_threads");
   workerData.names.push("added in the worker");
   Atomics.add(workerData.counter, 0, 5);
   parentPort.postMessage(workerData.names.length);`,
  { eval: true, workerData: { names, counter } },
);
worker.on("message", (n: number) => {
  console.log(`the array was copied: the worker's has ${n} items, main's has ${names.length}`);
  console.log(`the SharedArrayBuffer was not: main reads ${Atomics.load(counter, 0)} after the worker added 5`);
});
