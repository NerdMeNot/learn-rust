const parsed = JSON.parse('"not a number"') as number; // `as` makes the checker believe you
console.log("typeof parsed:", typeof parsed);
console.log("parsed + 1:", parsed + 1);
