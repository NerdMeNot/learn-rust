function get(a: Int32Array, i: number): string {
  return i < a.length ? `Some(${a[i]})` : "None";
}

const prices = new Int32Array([10, 20, 30]);
// there are no pointers: the nearest thing is a byte offset into an ArrayBuffer
const second = new DataView(prices.buffer).getInt32(4, true);
console.log(`second, read by offset: ${second}`);
console.log(`get(1): ${get(prices, 1)}`);
console.log(`get(9): ${get(prices, 9)}`);
