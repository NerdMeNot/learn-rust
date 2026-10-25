import { add, stats, Counter, countWords, type Stats } from './index.js'

const s: Stats = stats([3, 1, 2])
const c = new Counter(5)
console.log(add(2, 3), s.largest, c.increment(), await countWords('a b c'))
