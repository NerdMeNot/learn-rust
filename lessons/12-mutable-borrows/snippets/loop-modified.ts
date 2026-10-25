// Remove 3 (the second-to-last item) while looping: no error, and 4 is skipped.
let items = [1, 2, 3, 4];
let seen: number[] = [];
for (const x of items) {
  seen.push(x);
  if (x === 3) items.splice(items.indexOf(3), 1);
}
console.log("remove in a loop: saw", seen, "list is", items);

// Add 99 when the loop sees 2: no error, and the loop reaches 99 too.
items = [1, 2, 3, 4];
seen = [];
for (const x of items) {
  seen.push(x);
  if (x === 2) items.push(99);
}
console.log("add in a loop:    saw", seen, "list is", items);
