interface User {
  name: string;
  age: number;
  admin: boolean;
}
const ana: User = { name: "ana", age: 30, admin: false };
const older = { ...ana, age: 31 }; // copies the fields, one level deep
console.log(ana.age, older.age, older.name);

function connect(opts: { retries?: number; verbose?: boolean } = {}) {
  const { retries = 3, verbose = false } = opts;
  console.log({ retries, verbose });
}
connect();
connect({ verbose: true });
