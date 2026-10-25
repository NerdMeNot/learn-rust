interface Speaker {
  speak(): string;
}

function introduce(s: Speaker): void {
  console.log(s.speak());
}

// A bound, like `T: Speaker` in Rust.
function twice<T extends Speaker>(s: T): string {
  return s.speak() + s.speak();
}

// No class and no declaration: an object with the right shape is a Speaker.
introduce({ speak: () => "beep" });
console.log(twice({ speak: () => "beep" }));
