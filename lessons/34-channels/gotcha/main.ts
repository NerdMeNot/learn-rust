import { Worker, isMainThread, parentPort } from "node:worker_threads";

if (isMainThread) {
  const worker = new Worker(__filename);
  worker.on("message", (msg: number[]) => console.log(msg));
} else {
  const msg = [1, 2];
  parentPort!.postMessage(msg);
  msg.push(3); // change it after sending
}
