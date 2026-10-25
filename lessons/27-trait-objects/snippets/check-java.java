import java.util.ArrayList;
import java.util.List;

interface Shape {
    int area();
}

sealed interface Kind permits Circle, Tri {}
record Circle(int r) implements Kind {}
record Tri(int b) implements Kind {}

class Square implements Shape {
    int side = 3;
    public int area() { return side * side; }
}

class Rect implements Shape {
    public int area() { return 24; }
}

interface Visitable {
    <R> R accept(java.util.function.Function<Integer, R> f);
}

abstract class Base {
    protected int id = 7;
    abstract int area();
}

public class Main {
    static <T extends Shape> int twice(T s) { return 2 * s.area(); }

    static String kind(Kind k) {
        return switch (k) {
            case Circle c -> "circle " + c.r();
            case Tri t -> "tri " + t.b();
        };
    }

    public static void main(String[] args) {
        List<Shape> shapes = List.of(new Square(), new Rect());
        // instanceof with a binding
        if (!(shapes.get(0) instanceof Square sq) || sq.side != 3) throw new RuntimeException("instanceof");
        if (shapes.get(1) instanceof Square) throw new RuntimeException("a Rect is not a Square");
        // sealed interface + switch
        if (!kind(new Circle(2)).equals("circle 2") || !kind(new Tri(5)).equals("tri 5")) throw new RuntimeException("switch");
        // a null in the list is allowed (List.of refuses one, an ArrayList takes it)
        List<Shape> withNull = new ArrayList<>();
        withNull.add(new Square());
        withNull.add(null);
        if (withNull.size() != 2 || withNull.get(1) != null) throw new RuntimeException("null");
        // an interface can declare a generic method: <R> R accept(Visitor<R> v)
        Visitable v = new Visitable() {
            public <R> R accept(java.util.function.Function<Integer, R> f) { return f.apply(5); }
        };
        if (!v.accept(n -> "n=" + n).equals("n=5")) throw new RuntimeException("generic method");
        // an abstract class can hold a field
        Base b = new Base() { int area() { return id; } };
        if (b.area() != 7) throw new RuntimeException("abstract");
        // a generic bound
        if (twice(new Square()) != 18) throw new RuntimeException("generic");
        System.out.println("ok");
    }
}
