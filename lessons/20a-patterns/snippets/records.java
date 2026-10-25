public class Main {
    record Point(int x, int y) {}

    record Line(Point from, Point to) {}

    static String describe(Object o) {
        return switch (o) {
            // patterns nest, and are matched by position
            case Line(Point(var x1, var y1), Point(var x2, var y2)) when x1 == x2 ->
                "vertical line at x = " + x1;
            case Line(Point from, Point to) -> "line from " + from + " to " + to;
            case Point(int x, int y) -> "point " + x + "," + y;
            default -> "something else";
        };
    }

    public static void main(String[] args) {
        System.out.println(describe(new Line(new Point(1, 0), new Point(1, 5))));
        System.out.println(describe(new Line(new Point(0, 0), new Point(2, 3))));
        System.out.println(describe(new Point(4, 4)));
        System.out.println(describe("text"));
    }
}
