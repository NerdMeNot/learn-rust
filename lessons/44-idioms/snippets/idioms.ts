// 1. A branded type: a number the compiler treats as a different type. The brand is erased at run time.
type UserId = number & { readonly __brand: "UserId" };
const userId = (n: number) => n as UserId;

function loadUser(id: UserId): string {
  return `user #${id}`;
}
console.log(loadUser(userId(7)), `(typeof: ${typeof userId(7)})`);

// 2. Defaults with spread: later keys win.
const defaults = { host: "localhost", port: 8080, retries: 3 };
console.log({ ...defaults, port: 9000 });

