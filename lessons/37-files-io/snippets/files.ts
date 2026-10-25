import * as fs from "node:fs";
import { readFile } from "node:fs/promises";
import * as os from "node:os";
import * as path from "node:path";

try {
  fs.readFileSync("/no/such/file.txt", "utf8");
} catch (e) {
  const err = e as NodeJS.ErrnoException;
  console.log("sync:", err.code, "|", err.message);
}

const file = path.join(os.tmpdir(), "lr-fio-ts.txt");
fs.writeFileSync(file, Buffer.from([0x62, 0xff, 0x78, 0x0a]));
const text = fs.readFileSync(file, "utf8");
console.log("utf8 read of invalid bytes: no error, chars:", [...text.trimEnd()].map((c) => c.charCodeAt(0).toString(16)));

async function main() {
  try {
    await readFile("/no/such/file.txt");
  } catch (e) {
    console.log("promises:", (e as NodeJS.ErrnoException).code);
  }
  fs.unlinkSync(file);
  console.log("path.join(absolute):", path.join("/data", "/etc/hosts"), "| path.resolve:", path.resolve("/data", "/etc/hosts"));
}
main();
