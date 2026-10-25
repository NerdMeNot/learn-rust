interface User {
  name: string;
}

// with strictNullChecks, `undefined` must be part of the type to be returned
function findUser(id: number): User | undefined {
  return id === 1 ? { name: "ana" } : undefined;
}

const u = findUser(1);
if (u !== undefined) {
  console.log(`found ${u.name}`);
} else {
  console.log("no user");
}

// the short form: ?. gives undefined when there is no user
const found = findUser(2)?.name;
console.log(found ? `found ${found}` : "no user");
