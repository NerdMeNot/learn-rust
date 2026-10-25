console.log(parseInt("abc", 10) + 1);

function parsePort(s: string): number {
  const n = Number(s);
  if (!Number.isInteger(n) || n < 0 || n > 65535) throw new Error(`bad port: ${s}`);
  return n;
}

console.log(parsePort("8080"));
try {
  console.log(parsePort("80x"));
} catch (e) {
  console.log("caught:", e instanceof Error ? e.message : e);
}
