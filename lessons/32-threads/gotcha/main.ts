import { Worker } from "node:worker_threads";

const names = ["a", "b", "c"];
// the worker's code runs in a separate JS world
new Worker("console.log(names.length)", { eval: true });
