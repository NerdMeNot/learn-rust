import java.lang.management.ManagementFactory;
import java.util.ArrayList;
import java.util.List;

public class Main {
    static class Point {
        int x, y;
        Point(int x, int y) { this.x = x; this.y = y; }
    }

    static final com.sun.management.ThreadMXBean MX =
        (com.sun.management.ThreadMXBean) ManagementFactory.getThreadMXBean();

    static long allocated() {
        return MX.getThreadAllocatedBytes(Thread.currentThread().threadId());
    }

    static Object keep; // the objects stay reachable, so the JIT cannot drop them

    public static void main(String[] args) {
        int n = 1_000_000;

        long before = allocated();
        Point[] points = new Point[n];
        for (int i = 0; i < n; i++) points[i] = new Point(i, i);
        keep = points;
        System.out.println("1M Points in an array: " + (allocated() - before) / 1_000_000 + " MB");

        before = allocated();
        int[] ints = new int[n];
        for (int i = 0; i < n; i++) ints[i] = i + 1000;
        keep = ints;
        System.out.println("int[1M]:               " + (allocated() - before) / 1_000_000 + " MB");

        before = allocated();
        List<Integer> boxed = new ArrayList<>(n);
        for (int i = 0; i < n; i++) boxed.add(i + 1000);
        keep = boxed;
        System.out.println("List<Integer> of 1M:   " + (allocated() - before) / 1_000_000 + " MB");
    }
}
