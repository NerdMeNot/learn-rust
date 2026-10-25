const factor = 3;
const times = (x: number) => x * factor; // reads factor
console.log(times(5));

let count = 0;
const inc = () => { count += 1; };       // changes count
inc();
inc();
console.log(`count=${count}`);
