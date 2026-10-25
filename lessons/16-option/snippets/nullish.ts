function withOr(x: number | undefined): number {
  return x || 5;
}

function withNullish(x: number | undefined): number {
  return x ?? 5;
}

function textOr(x: string | undefined): string {
  return x || "none";
}

console.log("0 || 5:", withOr(0));
console.log("0 ?? 5:", withNullish(0));
console.log("undefined ?? 5:", withNullish(undefined));
console.log('"" || "none":', textOr(""));
console.log("NaN || 5:", withOr(NaN));
console.log("null == undefined:", null == undefined);
console.log("null === undefined:", null === undefined);
console.log("typeof null:", typeof null);
console.log("typeof undefined:", typeof undefined);
