const items: number[] = [1, 2, 3];
const view: readonly number[] = items; // `view` has no push(): read-only through this name

items.push(4); // the other name can still change it
console.log("view after items.push(4):", view);
