// A Result you write yourself: a union the compiler narrows.
type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };

function parseAge(s: string): Result<number, string> {
  if (!/^\d+$/.test(s)) return { ok: false, error: `not a number: ${s}` };
  return { ok: true, value: Number(s) };
}

for (const text of ["36", "abc"]) {
  const r = parseAge(text);
  if (r.ok) {
    console.log("ok", r.value);
  } else {
    console.log("error", r.error); // r.value does not exist on this branch
  }
}
