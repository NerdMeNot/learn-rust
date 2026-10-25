import { readFileSync } from 'node:fs'

const file = process.argv[2] ?? 'target/wasm32-unknown-unknown/release/adder.wasm'

let memory // set after instantiation; host_log reads the text from it
const imports = {
  env: {
    host_log(ptr, len) {
      console.log('  [from wasm]', new TextDecoder().decode(new Uint8Array(memory.buffer, ptr, len)))
    },
  },
}

const module = await WebAssembly.compile(readFileSync(file))
console.log('imports:', WebAssembly.Module.imports(module).map(i => `${i.module}.${i.name} (${i.kind})`).join(', '))
console.log('exports:', WebAssembly.Module.exports(module).map(e => `${e.name} (${e.kind})`).join(', '))
const { exports: w } = await WebAssembly.instantiate(module, imports)
memory = w.memory

// Numbers cross as numbers. An i64 arrives in JavaScript as a BigInt.
console.log('add(2, 3) =', w.add(2, 3))
console.log('fib(50) =', w.fib(50), typeof w.fib(50))
console.log('add(2147483647, 1) =', w.add(2147483647, 1))

// Text: copy the UTF-8 bytes into memory the module gave us, call, copy the answer out, free both.
function shout(text) {
  const input = new TextEncoder().encode(text)
  const inPtr = w.alloc(input.length)
  new Uint8Array(memory.buffer, inPtr, input.length).set(input)
  const lenPtr = w.alloc(4) // room for one usize (4 bytes on wasm32)
  const outPtr = w.shout(inPtr, input.length, lenPtr)
  const outLen = new Uint32Array(memory.buffer, lenPtr, 1)[0]
  const out = new TextDecoder().decode(new Uint8Array(memory.buffer, outPtr, outLen))
  w.dealloc(outPtr, outLen)
  w.dealloc(lenPtr, 4)
  w.dealloc(inPtr, input.length)
  return out
}
console.log(shout('straße'))

// An array: write four i32 values into memory, pass the address.
const nums = Int32Array.of(10, 20, 30, 40)
const numsPtr = w.alloc(nums.byteLength)
new Int32Array(memory.buffer, numsPtr, nums.length).set(nums)
console.log('sum =', w.sum(numsPtr, nums.length))
w.dealloc(numsPtr, nums.byteLength)

// A panic in Rust is a trap: the call throws.
try {
  w.divide(1, 0)
} catch (e) {
  console.log(e.name + ':', e.message)
}

// A view onto memory goes stale when the module's memory grows (MDN: grow() detaches the old buffer).
const view = new Uint8Array(memory.buffer)
console.log('view covers all of memory:', view.length === memory.buffer.byteLength)
const big = w.alloc(8 * 1024 * 1024)
console.log('old view length after alloc(8 MiB):', view.length)
console.log('a fresh view covers the new memory:', new Uint8Array(memory.buffer).length > 8 * 1024 * 1024)
w.dealloc(big, 8 * 1024 * 1024)
