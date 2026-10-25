import { spawnSync } from "node:child_process";

// argv: the first two are node and the script
console.log(`${process.argv.length - 2} arg(s) after the program name`);

// env values are string | undefined
const port = process.env.RBC_PORT ?? "8080";
console.log(`port ${port}`);
const missing = process.env.RBC_MISSING;
console.log(missing !== undefined ? `RBC_MISSING=${missing}` : "RBC_MISSING: not set");

// performance.now() is a monotonic clock in milliseconds
const start = performance.now();
// no blocking sleep in std: Atomics.wait is the honest stand-in (setTimeout can fire up to 1ms early)
Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 20);
const took = performance.now() - start;
console.log(`slept at least 20ms: ${took >= 20}`);

// run another program and capture what it prints
const out = spawnSync("echo", ["hello from echo"], { encoding: "utf8" });
process.stdout.write(out.stdout);
console.log(`exit ok: ${out.status === 0}`);
