type Light = "red" | "yellow" | "green";

function action(light: Light): string {
  switch (light) {
    case "red":
      return "stop";
    case "green":
      return "go";
    default: {
      const missing: never = light;
      return missing;
    }
  }
}

console.log(action("red"));
