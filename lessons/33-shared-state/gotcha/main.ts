import { Worker, isMainThread, workerData } from "node:worker_threads";
import { once } from "node:events";

const results: number[] = [];

async function main(): Promise<void> {
  const workers = [0, 1, 2].map((i) => new Worker(__filename, { workerData: i }));
  await Promise.all(workers.map((w) => once(w, "exit")));
  console.log(results);
}

if (isMainThread) {
  main();
} else {
  results.push(workerData * 10); // no lock!
}
