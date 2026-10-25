interface User {
  name: string;
  age: number;
}
const u = JSON.parse('{"name":"ana"}') as User; // the compiler believes you
console.log(u.age + 1, typeof u.age);
