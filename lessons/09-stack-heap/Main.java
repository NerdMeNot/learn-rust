public class Main {
    static class Point {
        int x, y;

        Point(int x, int y) {
            this.x = x;
            this.y = y;
        }

        @Override
        public String toString() {
            return "(" + x + ", " + y + ")";
        }
    }

    public static void main(String[] args) {
        Point a = new Point(1, 2);
        Point b = a; // b is another name for the same object
        b.x = 99;
        System.out.println("a = " + a);
        System.out.println("b = " + b);

        // To get a separate point, you build one.
        Point c = new Point(a.x, a.y);
        c.x = 7;
        System.out.println("a after c.x = 7: " + a);
    }
}
