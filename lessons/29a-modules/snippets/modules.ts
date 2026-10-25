import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const root = mkdtempSync(join(tmpdir(), "tsmod-"));
writeFileSync(
  join(root, "shop.ts"),
  "const rate = 20;\nexport function total(q: number): number {\n  return q * 10 - (q * 10 * rate) / 100;\n}\n",
);
writeFileSync(join(root, "index.ts"), 'export * from "./shop";\n');

function compile(label: string, mainSource: string): void {
  console.log(label);
  writeFileSync(join(root, "main.ts"), mainSource);
  const flags = ["--strict", "--module", "nodenext", "--types", "node"];
  const out = spawnSync("tsc", [...flags, "main.ts"], { cwd: root, encoding: "utf8" });
  console.log(out.status === 0 ? "  ok" : out.stdout.trim());
}

compile("1. import the exported name", 'import { total } from "./shop";\nconsole.log(total(12));\n');
compile("2. import a name that is not exported", 'import { rate } from "./shop";\nconsole.log(rate);\n');
compile("3. import it through an index.ts that says export *", 'import { total } from "./index";\nconsole.log(total(12));\n');

// Two files that import each other, run as ES modules.
const cycle = join(root, "cycle");
mkdirSync(cycle);
writeFileSync(join(cycle, "package.json"), '{ "type": "module" }\n');

function runCycle(label: string, a: string, b: string): void {
  console.log(label);
  writeFileSync(join(cycle, "a.ts"), a);
  writeFileSync(join(cycle, "b.ts"), b);
  const flags = ["--strict", "--module", "nodenext", "--target", "es2022", "--types", "node"];
  spawnSync("tsc", [...flags, "a.ts", "b.ts"], { cwd: cycle });
  const out = spawnSync("node", ["a.js"], { cwd: cycle, encoding: "utf8" });
  const lines = (out.stdout + out.stderr).trim().split("\n");
  console.log("  " + (lines.find((l) => l.includes("Error")) ?? lines[0]));
}

runCycle(
  "4. a imports b and b imports a; b uses f only when called",
  'import { g } from "./b.js";\nexport function f(): number {\n  return 1;\n}\nconsole.log(g());\n',
  'import { f } from "./a.js";\nexport function g(): number {\n  return f() + 1;\n}\n',
);
runCycle(
  "5. the same, but b calls a's f while it loads",
  'import { g } from "./b.js";\nexport const f = (): number => 1;\nconsole.log(g);\n',
  'import { f } from "./a.js";\nexport const g: number = f() + 1;\n',
);
