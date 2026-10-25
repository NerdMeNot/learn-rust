interface Speaker {
  speak(): string;
}

class Robot { // never says "implements Speaker"
  constructor(private id: number) {}
  speak(): string {
    return `beep ${this.id}`;
  }
}

function introduce(s: Speaker): void {
  console.log(s.speak());
}

introduce(new Robot(7));
