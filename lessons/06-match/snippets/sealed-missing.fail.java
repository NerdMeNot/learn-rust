public class Main {
    sealed interface Shape permits Circle, Rect {}
    record Circle(int r) implements Shape {}
    record Rect(int w, int h) implements Shape {}

    static int area(Shape s) {
        return switch (s) {
            case Circle c -> 3 * c.r() * c.r();
        };
    }

    public static void main(String[] args) {
        System.out.println(area(new Circle(2)));
    }
}
