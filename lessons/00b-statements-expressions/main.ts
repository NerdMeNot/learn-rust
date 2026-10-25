function clamp(x: number): number {
    if (x < 0) {
        return 0;
    }
    return x > 100 ? 100 : x;
}

const n = 7;
const label = n % 2 === 0 ? "even" : "odd";
const area = (() => {
    const w = 3;
    const h = 4;
    return w * h;
})();
console.log(`${n} is ${label}`);
console.log(`area: ${area}`);
console.log("clamp:", clamp(-5), clamp(50), clamp(250));
