import { parseArgs } from "node:util";

const { values, positionals } = parseArgs({
  allowPositionals: true,
  options: {
    verbose: { type: "boolean", short: "v" },
    priority: { type: "string", short: "p", default: "2" },
  },
});

if (positionals[0] === "add" && positionals.length === 2) {
  // parseArgs gives strings: turning one into a number, and refusing "abc", is our job.
  const priority = Number(values.priority);
  if (!Number.isInteger(priority)) {
    console.error(`invalid priority: ${values.priority}`);
    process.exit(2);
  }
  console.log(`added "${positionals[1]}" with priority ${priority}`);
  if (values.verbose) console.log("(verbose mode is on)");
} else if (positionals[0] === "list") {
  console.log("nothing to list yet");
} else {
  console.error("usage: todo [-v] add [-p N] TEXT | list");
  process.exit(2);
}
