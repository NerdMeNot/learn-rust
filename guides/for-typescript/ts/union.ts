function show(x: string | number) {
  console.log(typeof x === "string" ? x.toUpperCase() : x.toFixed(2));
}
show("ana");
show(2);
