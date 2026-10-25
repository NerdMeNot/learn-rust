import { spawnSync } from "node:child_process";

// the timeout option: when it passes, Node kills the child (SIGTERM by default)
const slow = spawnSync("sleep", ["60"], { timeout: 200 });
console.log("timeout:", (slow.error as NodeJS.ErrnoException).code, "status", slow.status, "signal", slow.signal);

// a child ended by a signal has status null and a signal name
const killed = spawnSync("sh", ["-c", "kill -9 $$"]);
console.log("kill -9: status", killed.status, "signal", killed.signal);
