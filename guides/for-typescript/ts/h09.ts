async function fetchName(): Promise<string> {
  console.log("fetching");
  return "ana";
}

async function main() {
  const p = fetchName(); // the body has already started
  console.log("created");
  console.log(await p);
}
main();
