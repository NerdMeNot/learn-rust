const log: string[] = [];
const yieldToLoop = () => new Promise<void>((resolve) => setImmediate(resolve));

async function heartbeat() {
  for (let i = 1; i <= 3; i++) {
    log.push(`tick ${i}`);
    await yieldToLoop(); // let the event loop run something else
  }
}

async function main() {
  const hb = heartbeat(); // a promise runs when it is created: tick 1 is logged here

  // CPU-bound work, called straight from an async function
  let total = 0;
  for (let n = 0; n < 1_000_000; n++) total += n % 7;
  log.push(`sum done, total ${total}`);

  await hb;
  console.log(log.join("\n"));
}

main();
