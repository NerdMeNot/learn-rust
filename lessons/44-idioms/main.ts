class Port {
  // A private constructor: outside the class, `new Port(0)` does not compile.
  private constructor(readonly number: number) {}

  static parse(text: string): Port {
    if (!/^\d+$/.test(text)) throw new Error("not a number");
    const n = Number(text);
    if (n < 1 || n > 65535) throw new Error(`${n} is outside 1-65535`);
    return new Port(n);
  }
}

function listen(port: Port): void {
  console.log(`listening on ${port.number}`);
}

for (const text of ["8080", "0", "http"]) {
  try {
    listen(Port.parse(text));
  } catch (e) {
    console.log(`"${text}": ${(e as Error).message}`);
  }
}
