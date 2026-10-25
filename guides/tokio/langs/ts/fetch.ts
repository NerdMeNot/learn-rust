const sleep = (ms: number) => new Promise<void>((resolve) => setTimeout(resolve, ms));

async function fetchData(name: string, ms: number): Promise<string> {
  await sleep(ms);
  if (ms >= 500) console.log("(slow finished anyway, nobody was waiting)");
  return `${name} done after ${ms}ms`;
}

async function main() {
  // a promise is already running when you get it
  const users = fetchData("users", 30);
  const orders = fetchData("orders", 10);
  console.log(await users);
  console.log(await orders);

  const [left, right] = await Promise.all([fetchData("left", 20), fetchData("right", 20)]);
  console.log(left, "/", right);

  const deadline = new Promise<string>((resolve) => setTimeout(() => resolve("slow call timed out"), 50));
  console.log(await Promise.race([fetchData("slow", 500), deadline]));
}

main();
