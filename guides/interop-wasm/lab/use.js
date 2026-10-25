const { shout, sum, parse, Counter } = require('./bindgen/pkg/greeter.js')

console.log(shout('straße'))
console.log('sum =', sum(Int32Array.of(10, 20, 30, 40)))
console.log('parse(" 42 ") =', parse(' 42 '))
try {
  parse('4x2')
} catch (e) {
  console.log(`${e.name}: ${e.message}`)
}
const c = new Counter()
console.log(c.bump(), c.bump())
c.free() // Rust memory is not garbage collected until you free it (or a FinalizationRegistry runs)
