function logged<T extends (...args: any[]) => any>(fn: T, context: ClassMethodDecoratorContext) {
    console.log(`decorating ${String(context.name)}`);
    return function (this: unknown, ...args: any[]) {
        console.log(`calling ${String(context.name)}`);
        return fn.apply(this, args);
    };
}

console.log("before the class");

class Greeter {
    @logged
    hello(): string {
        return "hi";
    }
}

console.log("after the class");
console.log(new Greeter().hello());
