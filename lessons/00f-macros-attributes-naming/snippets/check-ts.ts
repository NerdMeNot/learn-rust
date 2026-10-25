function same(got: string, want: string): void {
  if (got !== want) throw new Error(`got [${got}], want [${want}]`);
}

const n = 42;
const pi = 3.14159;
same(String(n).padStart(5), "   42");
same(String(n).padEnd(5), "42   ");
same(String(n).padStart(5, "0"), "00042");
same(pi.toFixed(2), "3.14");
same(pi.toFixed(3).padStart(8), "   3.142");
same((255).toString(16), "ff");
same("0x" + (255).toString(16), "0xff");
same("Ada".padStart(10), "       Ada");
same("Ada".padEnd(10), "Ada       ");
same("Lovelace".slice(0, 3), "Lov");
same(`${n} ${"Ada"}`, "42 Ada");
console.log("ok");
