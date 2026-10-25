import { Worker, isMainThread, parentPort, workerData } from "node:worker_threads";

// Run this same file in a worker. `data` is copied to it, and the answer is copied back.
function inWorker<T>(data: unknown): Promise<T> {
  return new Promise((resolve) => {
    new Worker(__filename, { workerData: data }).on("message", resolve);
  });
}

if (isMainThread) {
  (async () => {
    const words = ["alpha", "beta"];

    const back = await inWorker<string[]>({ kind: "echo", words });
    console.log(`moved there and back: [${back.map((w) => JSON.stringify(w)).join(", ")}]`);

    // two workers, each with its own copy of the list
    const [a, b] = await Promise.all([
      inWorker<number>({ kind: "len", words, i: 0 }),
      inWorker<number>({ kind: "len", words, i: 1 }),
    ]);
    console.log(`lengths read in parallel: ${a} and ${b}`);
  })();
} else {
  const { kind, words, i } = workerData;
  parentPort!.postMessage(kind === "echo" ? words : words[i].length);
}
