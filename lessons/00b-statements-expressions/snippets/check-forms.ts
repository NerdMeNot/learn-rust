function log(msg: string): void {
    console.log("log: " + msg);
}

const n = 7;

const size = n > 5 ? "big" : "small";
console.log("ternary:", size);

const area = (() => {
    const w = 3;
    const h = 4;
    return w * h;
})();
console.log("block:", area);

let name: string;
const code: number = 2;
switch (code) {
    case 1:
        name = "one";
        break;
    case 2:
        name = "two";
        break;
    default:
        name = "many";
}
console.log("switch:", name);

let a = 1;
let b = 2;
[a, b] = [b, a];
console.log("swap:", a, b);

log("first");
const c = 2;
console.log("comma:", c);

const addOne = (x: number) => {
    const y = x + 1;
    return y;
};
console.log("closure:", addOne(41), ((x: number) => x + 1)(41));
