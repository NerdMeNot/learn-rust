import { MessageChannel } from "node:worker_threads";

const { port1, port2 } = new MessageChannel();
let seen = 0;
port2.on("message", (m: Uint8Array) => {
  console.log(`receiver sees [${m}]`);
  if (++seen === 2) port2.close();
});

const copied = new Uint8Array([1, 2, 3]);
port1.postMessage(copied); // structured clone: the receiver gets a copy
copied[0] = 99; // too late to matter
console.log(`sent a copy; sender still has [${copied}]`);

const moved = new Uint8Array([4, 5, 6]);
port1.postMessage(moved, [moved.buffer]); // transfer list: the buffer moves
console.log(`transferred the buffer; sender's byteLength is now ${moved.byteLength}`);
