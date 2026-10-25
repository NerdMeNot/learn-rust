function kindOf(day: number): string {
  switch (day) {
    case 1: case 2: case 3: case 4: case 5:
      return "weekday";
    case 6: case 7:
      return "weekend";
    default: // anything else
      return "not a day";
  }
}

console.log(kindOf(6));
