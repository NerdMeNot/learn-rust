import java.util.List;

public class Main {
    /** A point on a grid. */
    record Point(int x, int y) {}

    public static void main(String[] args) {
        Point p = new Point(3, 4); // a line comment
        List<Integer> items = List.of(1, 2, 3);
        if (items.size() != 3) throw new IllegalStateException("items should have 3 elements");
        System.out.println("point: Point { x: " + p.x() + ", y: " + p.y() + " }");
        System.out.println("items: " + items);
        System.out.println(String.format("row: [%5d] [%-5d] [%05d]", 42, 42, 42));
        String pi = String.format("%.2f", 3.14159);
        System.out.println("pi: " + pi + " {done}");
    }
}
