public class Main {
    record Point(int x, int y) {}

    public static void main(String[] args) {
        // a record pattern takes the record apart
        Point p = new Point(3, -2);
        if (p instanceof Point(int x, int y)) {
            System.out.println("x = " + x + ", y = " + y);
        }

        String abc = "abc";
        for (int i = 0; i < abc.length(); i++) {
            System.out.println(i + ": " + abc.charAt(i));
        }

        String[] kv = "mode=fast".split("=", 2);
        if (kv.length == 2) {
            System.out.println(kv[0] + " -> " + kv[1]);
        }

        // an array is read by index
        int[] a = {10, 20, 30, 40};
        System.out.println("first = " + a[0] + ", last = " + a[a.length - 1]);
    }
}
