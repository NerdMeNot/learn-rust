interface Team {
  name: string;
  members: string[];
}

const a: Team = { name: "red", members: ["ana"] };
const b = a; // two names for one object
b.name = "blue";
console.log("a.name after b = a, b.name = 'blue':", a.name);

const c = { ...a, name: "green" }; // a new object, but a shallow copy
c.members.push("bob");
console.log("a.members:", a.members.join(", "));
console.log("same array object:", a.members === c.members);
