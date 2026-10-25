interface Person {
  fullName: string;
  age: number;
}

const cy = JSON.parse('{ "fullName": "Cy" }') as Person; // no "age"
console.log(cy);
console.log(typeof cy.age, cy.age + 1);
