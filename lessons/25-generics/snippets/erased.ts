function identity<T>(value: T): T {
  return value;
}

// The compiled JavaScript has no trace of T.
console.log(identity.toString());
console.log(identity<number>(5), identity<string>("five"));
