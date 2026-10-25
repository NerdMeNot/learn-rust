import { add, addChecked, stats, parsePort, Counter, xorInPlace, xorCopy, countWords, threadId, threadIdAsync, countUp } from './index.js'

console.log('add(2, 3)           =', add(2, 3))
console.log('add(2 ** 31, 1)     =', add(2 ** 31, 1))
console.log('add(1.5, 2)         =', add(1.5, 2))
try { addChecked(2147483647, 1) } catch (e) { console.log('addChecked        :', e.code, '|', e.message) }
try { add('1', 2) } catch (e) { console.log('add("1", 2)        :', e.code, '|', e.message) }

console.log('stats([3, 1, 2])    =', stats([3, 1, 2]))
console.log('stats([])           =', stats([]))
console.log('parsePort(" 8080 ") =', parsePort(' 8080 '))
try { parsePort('99999') } catch (e) { console.log('parsePort("99999")  :', e instanceof Error, e.code, '|', e.message) }

const counter = new Counter(5)
counter.increment()
console.log('Counter             =', counter.count, counter instanceof Counter)

const shared = Buffer.from([1, 2, 3])
xorInPlace(shared, 0xff)
console.log('xorInPlace(buffer)  ->', shared)
const copied = [1, 2, 3]
console.log('xorCopy(array)      ->', xorCopy(copied, 0xff), 'and the array is still', copied)

const pending = countWords('one two  three')
console.log('countWords          -> Promise?', pending instanceof Promise)
console.log('countWords          =', await pending)
console.log('same thread twice?  ', threadId() === threadId(), '| async on another thread?', threadId() !== (await threadIdAsync()))

await new Promise((resolve) => countUp(3, (n) => { console.log('tick', n); if (n === 3) resolve() }))
