interface Meters {
  value: number;
}
interface Feet {
  value: number;
}

function describe(distance: Meters): string {
  return `${distance.value} meters`;
}

const altitude: Feet = { value: 1200 };
console.log(describe(altitude));
