class PointClass {
    private final int x;
    private final int y;

    PointClass(int x, int y) {
        this.x = x;
        this.y = y;
    }

    int getX() { return x; }
    int getY() { return y; }
}

record Point(int x, int y) {}

public class Records {
    public static void main(String[] args) {
        System.out.println(new PointClass(1, 2).equals(new PointClass(1, 2)));
        System.out.println(new Point(1, 2).equals(new Point(1, 2)));
        System.out.println(new Point(1, 2));
    }
}
