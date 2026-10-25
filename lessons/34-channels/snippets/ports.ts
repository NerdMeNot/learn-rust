import { once } from "node:events";
import { MessageChannel, receiveMessageOnPort } from "node:worker_threads";

class Point {
  constructor(public x: number) {}
}

function tryPost(label: string, send: () => void) {
  try {
    send();
    console.log(`${label}: sent`);
  } catch (e) {
    console.log(`${label}: ${(e as Error).name}`);
  }
}

const { port1, port2 } = new MessageChannel();

// receiveMessageOnPort is a synchronous poll, like try_recv: it never waits
console.log("empty port:", receiveMessageOnPort(port2));
port1.postMessage("hello");
console.log("after a post:", receiveMessageOnPort(port2)?.message);

// A SharedArrayBuffer is shared, not copied: the memory is the same (Rust's Arc)
const shared = new SharedArrayBuffer(4);
port1.postMessage(shared);
new Uint8Array((receiveMessageOnPort(port2)!.message as SharedArrayBuffer))[0] = 7;
console.log("receiver wrote 7, sender sees", new Uint8Array(shared)[0]);

// Only some types can be transferred; everything else is copied or refused
tryPost("transfer an array", () => port1.postMessage([1, 2, 3], [[1, 2, 3] as any]));
tryPost("send a function", () => port1.postMessage({ f() {} }));
port1.postMessage(new Point(5));
const got = receiveMessageOnPort(port2)!.message;
console.log("a class instance arrives as a plain object:", got instanceof Point, got);

// close() on either port closes both
const closes = Promise.all([once(port1, "close"), once(port2, "close")]);
port1.ref(); // keep the process alive until both events have arrived
port2.ref();
port1.close();
closes.then(() => console.log("port1.close() fired a close event on both ports"));
