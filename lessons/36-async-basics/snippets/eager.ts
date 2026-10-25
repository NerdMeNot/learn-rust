async function fetchName(): Promise<string> {
  console.log("  fetchName body runs");
  return "ferris";
}

async function main(): Promise<void> {
  console.log("calling fetchName()");
  const promise = fetchName();
  console.log("called it; the body has already run");
  console.log("name:", await promise);

  // the executor function of `new Promise` also runs at once
  const p = new Promise<number>((resolve) => {
    console.log("  executor runs");
    resolve(1);
  });
  console.log("promise built");
  await p;

  // to delay the work, hold a function that starts it
  const later = () => fetchName();
  console.log("thunk built; nothing has run for it");
  await later();

  // `await` always hands control back to the caller, even for a promise that is already resolved
  const order: string[] = [];
  async function child(): Promise<void> {
    order.push("child body");
    await Promise.resolve();
    order.push("child after its await");
  }
  const done = child();
  order.push("caller continues");
  await done;
  console.log(order.join(" | "));
}

main();
