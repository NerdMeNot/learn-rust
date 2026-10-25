public class Main {
    sealed interface Shape permits Circle, Rect, Triangle {}
    record Circle(double r) implements Shape {}
    record Rect(double w, double h) implements Shape {}
    record Triangle(double b, double h) implements Shape {}   // new!

    static double area(Shape s) {
        return switch (s) {
            case Circle c -> 3.14 * c.r() * c.r();
            case Rect r -> r.w() * r.h();
        };
    }

    public static void main(String[] args) {
        System.out.println(area(new Triangle(2.0, 3.0)));
    }
}
