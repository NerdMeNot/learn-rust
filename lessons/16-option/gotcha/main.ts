interface User {
  name: string;
}

function findUser(id: number): User | undefined {
  return id === 1 ? { name: "ana" } : undefined;
}

const u = findUser(2);
console.log(u.name);
