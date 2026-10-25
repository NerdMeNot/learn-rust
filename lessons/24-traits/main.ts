// an abstract class can carry a default method, like a trait
abstract class Speaker {
  abstract speak(): string;

  shout(): string { // default method
    return this.speak().toUpperCase();
  }
}

class Dog extends Speaker {
  speak(): string {
    return "woof";
  }
}

class Robot extends Speaker {
  constructor(private id: number) {
    super();
  }
  speak(): string {
    return `beep ${this.id}`;
  }
}

function introduce(s: Speaker): void {
  console.log(`${s.speak()} / ${s.shout()}`);
}

introduce(new Dog());
introduce(new Robot(7));
