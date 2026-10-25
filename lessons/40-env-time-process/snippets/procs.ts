import { execFileSync, spawnSync } from "node:child_process";

// process.argv holds the node binary, then the script, then the arguments
console.log("process.argv.length:", process.argv.length);

// arguments are an array: no shell
const echo = spawnSync("echo", ["a  b", "$HOME", "one; echo two"], { encoding: "utf8" });
console.log("output:", JSON.stringify(echo.stdout));

// input is written to the child's stdin and the pipe is closed for you
console.log("sort printed:", JSON.stringify(spawnSync("sort", { input: "pear\napple\n", encoding: "utf8" }).stdout));

// spawnSync reports a failing program as a status; execFileSync throws
console.log("false status:", spawnSync("false").status);
try {
  execFileSync("false");
} catch (e) {
  console.log("execFileSync throws:", (e as Error).message);
}

// a missing variable is typed string | undefined
const port: string | undefined = process.env.RBC_MISSING;
console.log("process.env.RBC_MISSING:", port);

// process.env is an ordinary object you can assign to
process.env.RBC_MODE = "fast";
console.log("process.env.RBC_MODE:", process.env.RBC_MODE);

// performance.now() is monotonic; Date.now() is the wall clock
const start = performance.now();
setTimeout(() => {
  // Node's timers can fire a little early, so only "time passed" is asserted
  console.log("elapsed > 0:", performance.now() - start > 0);
  setTimeout(() => console.log("a pending timer ran"), 50);
  console.log("calling process.exit");
  process.exit(0);
}, 20);
