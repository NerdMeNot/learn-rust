async function task(): Promise<void> {
  console.log("task body runs");
}

async function main(): Promise<void> {
  console.log("start");
  const promise = task(); // the body has already run, up to its first await
  console.log("task created");
  await promise;
  console.log("end");
}

main();
