// Backs the Java cells of the tables in lesson.mdx. Never shown.
import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        List<Integer> v = new ArrayList<>(List.of(1, 2, 3, 4));
        v.removeIf(x -> x % 2 == 0);
        if (!v.equals(List.of(1, 3))) throw new AssertionError(v);
        var it = v.iterator();
        while (it.hasNext()) {
            if (it.next() == 1) it.remove();
        }
        if (!v.equals(List.of(3))) throw new AssertionError(v);
        // adding while looping throws; removing the second-to-last item does not
        List<Integer> w = new ArrayList<>(List.of(1, 2, 3, 4));
        try {
            for (int x : w) {
                if (x == 2) w.add(99);
            }
            throw new AssertionError("no exception");
        } catch (java.util.ConcurrentModificationException e) {
            // expected
        }
        List<Integer> q = new ArrayList<>(List.of(1, 2, 3, 4));
        List<Integer> seen = new ArrayList<>();
        for (int x : q) {
            seen.add(x);
            if (x == 3) q.remove(Integer.valueOf(3));
        }
        if (!q.equals(List.of(1, 2, 4))) throw new AssertionError(q);
        if (!seen.equals(List.of(1, 2, 3))) throw new AssertionError(seen);
        System.out.println("ok");
    }
}
