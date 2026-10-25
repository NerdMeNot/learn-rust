const prices = new Int32Array([10, 20, 30]);
try {
  new DataView(prices.buffer).getInt32(12, true); // byte offset 12 is just past the end
} catch (e) {
  console.log("caught " + (e as Error).name);
}
