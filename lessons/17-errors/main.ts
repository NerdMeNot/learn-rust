function parseAge(s: string): number {
  if (!/^\d+$/.test(s)) {
    throw new Error("invalid digit found in string");
  }
  return Number(s);
}

for (const input of ["42", "abc"]) {
  try {
    console.log(`age ${parseAge(input)}`);
  } catch (e) {
    console.log(`error: ${(e as Error).message}`);
  }
}
