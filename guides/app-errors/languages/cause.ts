try {
  try {
    JSON.parse("eighty");
  } catch (e) {
    throw new Error("reading bad.conf", { cause: e });
  }
} catch (e) {
  const err = e as Error;
  console.log(`${err.message}: ${(err.cause as Error).name}`);
}
