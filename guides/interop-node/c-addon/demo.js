const { add } = require('./add.node')

console.log('add(2, 3)       =', add(2, 3))
console.log('add(2 ** 31, 1) =', add(2 ** 31, 1))
console.log('add(1.5, 2)     =', add(1.5, 2))
try { add('1', 2) } catch (e) { console.log('add("1", 2)    :', e.name, '|', e.message) }
