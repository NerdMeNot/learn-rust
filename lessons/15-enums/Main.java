import java.util.List;

public class Main {
    sealed interface Shape permits Circle, Rect {}
    record Circle(double r) implements Shape {}
    record Rect(double w, double h) implements Shape {}

    static double area(Shape s) {
        return switch (s) {
            case Circle c -> 3.14 * c.r() * c.r();
            case Rect(double w, double h) -> w * h;
        };
    }

    public static void main(String[] args) {
        List<Shape> shapes = List.of(new Circle(1.0), new Rect(2.0, 3.0));
        for (var s : shapes) {
            System.out.println(area(s));
        }
    }
}
