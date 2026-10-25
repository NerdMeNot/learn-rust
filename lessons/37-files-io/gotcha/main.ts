import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";

const file = path.join(os.tmpdir(), "rbc-hello.txt");
fs.writeFileSync(file, "hello");

const fd = fs.openSync(file, "r");
try {
  const buf = Buffer.alloc(64);
  const n = fs.readSync(fd, buf);
  console.log(buf.toString("utf8", 0, n));
} finally {
  fs.closeSync(fd);
}
fs.unlinkSync(file);
