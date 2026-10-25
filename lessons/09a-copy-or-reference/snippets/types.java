import java.util.ArrayList;
import java.util.List;

public class Main {
    record Pair(List<Integer> items, int n) {}

    static class Point {
        int x;

        Point(int x) {
            this.x = x;
        }
    }

    static void report(String kind, boolean changed) {
        System.out.printf("%-8s %s%n", kind, changed
            ? "shares: the other name sees the change"
            : "copy:   the other name is untouched");
    }

    public static void main(String[] args) {
        int n = 1;
        int m = n;
        m = 99;
        report("int", n == 99);

        String s = "hi";
        String t = s;
        t += "!"; // builds a new String; s is untouched
        report("String", s.equals("hi!"));

        Point p = new Point(1);
        Point q = p;
        q.x = 99;
        report("object", p.x == 99);

        int[] arr = {1, 2};
        int[] arr2 = arr;
        arr2[0] = 99;
        report("int[]", arr[0] == 99);

        List<Integer> list = new ArrayList<>(List.of(1, 2));
        List<Integer> list2 = list;
        list2.set(0, 99);
        report("List", list.get(0) == 99);

        // A record is an object too: equals compares its components, == is identity
        List<Integer> shared = new ArrayList<>(List.of(1));
        Pair r1 = new Pair(shared, 1);
        Pair r2 = new Pair(shared, 1);
        shared.add(2);
        System.out.println("r1.equals(r2): " + r1.equals(r2) + ", r1 == r2: " + (r1 == r2) + ", r1.items(): " + r1.items());

        // Identity: == on objects asks "the same object?"
        Point r = new Point(99);
        System.out.println("p == q: " + (p == q) + "  (same object)");
        System.out.println("p == r: " + (p == r) + "  (same x, different object)");
        String a = new String("hi");
        System.out.println("a == \"hi\": " + (a == "hi") + ", a.equals(\"hi\"): " + a.equals("hi"));
    }
}
