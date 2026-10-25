interface Config {
  name: string;
  port: number;
  verbose: boolean;
}

const base: Config = { name: "service", port: 8080, verbose: false };
const test: Config = { ...base, port: 9000 };
console.log(base);
console.log(test);
