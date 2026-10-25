// Calls `add(2, 3)` in each module named on the command line. A module that imports
// wasi_snapshot_preview1 (Go's does) gets Node's WASI object first.
import { readFileSync } from 'node:fs'
import { WASI } from 'node:wasi'

for (const file of process.argv.slice(2)) {
  const bytes = readFileSync(file)
  const module = await WebAssembly.compile(bytes)
  const imports = WebAssembly.Module.imports(module)
  const wasi = new WASI({ version: 'preview1' })
  const needsWasi = imports.some(i => i.module === 'wasi_snapshot_preview1')
  const instance = await WebAssembly.instantiate(module, needsWasi ? wasi.getImportObject() : {})
  if (needsWasi) wasi.initialize(instance) // start the Go runtime; main is not run
  const names = [...new Set(imports.map(i => i.module))].join(', ') || 'none'
  console.log(`${file}: ${bytes.length} bytes, ${imports.length} imports (${names}), add(2, 3) = ${instance.exports.add(2, 3)}`)
}
