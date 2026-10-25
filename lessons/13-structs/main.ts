interface User {
  name: string;
  age: number;
  active: boolean;
}

const u: User = {
  name: "ana",
  age: 30,
  active: true,
};
console.log(`${u.name} is ${u.age}, active=${u.active}`);
console.log(u);
