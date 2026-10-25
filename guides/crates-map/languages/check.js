const wt = require('node:worker_threads');
console.log('TypeScript/Node:', typeof fetch, 'fetch,', typeof crypto.randomUUID, 'crypto.randomUUID,', typeof wt.Worker, 'worker_threads.Worker,', typeof require('node:test'), 'node:test,', typeof Intl, 'Intl');
