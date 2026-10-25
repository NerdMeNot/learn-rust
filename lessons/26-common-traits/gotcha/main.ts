class Version {
  constructor(
    readonly major: number,
    readonly minor: number,
  ) {}
}

const notes = new Map<Version, string>();
notes.set(new Version(1, 2), "stable");
console.log("found:", notes.get(new Version(1, 2)));
