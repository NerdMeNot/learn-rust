const names = ["ana", "bo"];
console.log(names[5]); // no error, no check
console.log(names[5]?.toUpperCase() ?? "none");

const port = 0;
console.log(port || 8080, port ?? 8080);
