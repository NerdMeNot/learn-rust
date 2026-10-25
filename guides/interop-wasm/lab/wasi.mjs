import { readFileSync, mkdirSync } from 'node:fs'
import { WASI } from 'node:wasi'

mkdirSync('sandbox', { recursive: true })
const wasi = new WASI({
  version: 'preview1',
  args: ['wasi-demo', 'one', 'two'], // args[0] is the program name
  env: { GREETING: 'hi' },
  preopens: { 'sandbox': 'sandbox' }, // guest path: host path
  stdin: 0,
})
const module = await WebAssembly.compile(readFileSync(process.argv[2]))
const instance = await WebAssembly.instantiate(module, wasi.getImportObject())
process.exitCode = wasi.start(instance)
