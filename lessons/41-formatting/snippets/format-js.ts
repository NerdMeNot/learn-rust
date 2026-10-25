const n = 1234567.5;
console.log((0.5).toFixed(0), (1.5).toFixed(0), (2.5).toFixed(0));
console.log(n.toLocaleString("en-US"), "|", n.toLocaleString("de-DE"));
console.log(JSON.stringify({ user: "ada", roles: ["admin", "dev"] }, null, 2));
