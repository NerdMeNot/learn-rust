import java.util.List;

public class Main {
    // Java has no built-in tuple, so a tiny record holds the two values
    record MinMax(int lo, int hi) {}

    static MinMax minMax(List<Integer> xs) {
        int lo = xs.get(0);
        int hi = xs.get(0);
        for (int x : xs) {
            if (x < lo) lo = x;
            if (x > hi) hi = x;
        }
        return new MinMax(lo, hi);
    }

    public static void main(String[] args) {
        var r = minMax(List.of(4, 9, 1, 7));
        System.out.println(r.lo() + " " + r.hi());

        int onlyHi = minMax(List.of(3, 5)).hi();
        System.out.println(onlyHi);
    }
}
