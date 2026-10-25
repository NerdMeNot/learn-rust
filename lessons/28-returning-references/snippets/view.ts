const data = new Uint8Array([1, 2, 3, 4]);

const view = data.subarray(0, 2); // shares memory with data
view[0] = 100;
console.log("data after view[0] = 100:", data);

const copy = data.slice(0, 2); // a new array
copy[1] = 200;
console.log("data after copy[1] = 200:", data);
