import { Worker } from "node:worker_threads";

const worker = new Worker(`while (true) {}`, { eval: true }); // never returns on its own
worker.terminate().then(() => {
  console.log("terminate() stopped the endless loop from outside");
});
