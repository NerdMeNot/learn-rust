function first(rows: number[][]): number[] {
  return rows[0]!; // the same array, not a copy
}

const rows = [[1], [2]];
first(rows).push(99);
console.log("rows:", JSON.stringify(rows));
