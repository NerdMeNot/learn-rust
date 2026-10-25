public class Main {
    sealed interface Shape permits Circle, Rect {}
    record Circle(double r) implements Shape {}
    record Rect(double w, double h) implements Shape {}

    // each case is its own type, so a method can ask for exactly one
    static double radius(Circle c) {
        return c.r();
    }

    public static void main(String[] args) {
        System.out.println("radius: " + radius(new Circle(1.0)));
    }
}
