import { Worker, isMainThread, parentPort } from "node:worker_threads";

if (isMainThread) {
  const worker = new Worker(__filename);
  // messages arrive in the order they were sent
  worker.on("message", (msg: string) => console.log(`got ${msg}`));
} else {
  for (let i = 1; i <= 3; i++) {
    parentPort!.postMessage(`job ${i}`); // the message is copied across
  }
} // the worker ends here and the program exits once its messages are handled
