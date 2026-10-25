const UNITS: Record<string, number> = { s: 1, m: 60, h: 3600 };

// Read text like "1h30m" or "90s" and return the number of seconds.
export function seconds(text: string): number {
  if (text === "") throw new Error("empty");
  let total = 0;
  let digits = "";
  for (const c of text) {
    if (c >= "0" && c <= "9") {
      digits += c;
      continue;
    }
    if (!(c in UNITS)) throw new Error(`unknown unit ${c}`);
    if (digits === "") throw new Error("no number");
    total += Number(digits) * UNITS[c];
    digits = "";
  }
  if (digits !== "") throw new Error("missing unit");
  return total;
}
