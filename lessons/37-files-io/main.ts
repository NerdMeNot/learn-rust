import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";
import * as readline from "node:readline";

async function main() {
  const file = path.join(os.tmpdir(), "rbc-notes.txt");

  // write a whole file at once (creates it, or empties it first)
  fs.writeFileSync(file, "milk\neggs\n");

  // add a line at the end
  fs.appendFileSync(file, "bread\n");

  // read the whole file into one string
  const all = fs.readFileSync(file, "utf8");
  console.log(`${all.length} chars`);

  // read line by line, without loading the whole file
  const lines = readline.createInterface({ input: fs.createReadStream(file) });
  let i = 1;
  for await (const line of lines) {
    console.log(`${i++}: ${line}`);
  }

  console.log(`file name: ${JSON.stringify(path.basename(file))}`);

  // a missing file throws; the error carries a code you can check
  try {
    fs.readFileSync("/no/such/file.txt", "utf8");
  } catch (e) {
    const err = e as NodeJS.ErrnoException;
    console.log(`error: ${err.message} (code ${err.code})`);
  }

  fs.unlinkSync(file);
}

main();
