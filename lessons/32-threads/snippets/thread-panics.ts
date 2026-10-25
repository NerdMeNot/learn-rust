import { Worker } from "node:worker_threads";

const worker = new Worker(`throw new Error("worker failed");`, { eval: true });
worker.on("error", (e: Error) => console.log(`main: the worker's error arrived as an event: ${e.message}`));
worker.on("exit", (code: number) => console.log(`main: worker exited with code ${code}`));
