async function loadCount(): Promise<number> {
  return 3;
}

async function report(): Promise<number> {
  const n: number = loadCount();
  return n + 1;
}

report().then((count) => console.log("count:", count));
