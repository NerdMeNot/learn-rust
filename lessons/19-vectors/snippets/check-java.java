import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Main {
    static void must(boolean ok, String what) {
        if (!ok) throw new AssertionError(what);
    }

    public static void main(String[] args) {
        List<Integer> v = new ArrayList<>(List.of(3, 1, 2));
        v.add(4);
        v.add(1, 5);
        must(v.equals(List.of(3, 5, 1, 2, 4)), "add");
        v.remove(1);
        must(v.equals(List.of(3, 1, 2, 4)), "remove(int) removes by index");
        v.addAll(List.of(7, 8));
        must(v.size() == 6 && !v.isEmpty() && v.contains(7), "size contains");
        v.removeIf(x -> x == 7);
        must(v.equals(List.of(3, 1, 2, 4, 8)), "removeIf");

        List<Integer> view = v.subList(1, 3);
        must(view.equals(List.of(1, 2)), "subList");
        v.set(1, 10);
        must(view.get(0) == 10, "subList is a view");
        view.set(0, 11);
        must(v.get(1) == 11, "writes through a subList reach the parent");

        Collections.sort(v);
        must(v.equals(List.of(2, 3, 4, 8, 11)), "sort");
        v.sort((a, b) -> b - a);
        must(v.equals(List.of(11, 8, 4, 3, 2)), "sort with comparator");
        List<Integer> w = new ArrayList<>(List.of(1, 2, 3, 4));
        w.subList(1, 3).clear();
        must(w.equals(List.of(1, 4)), "subList(...).clear() deletes a range");
        w.subList(0, 1).add(99);
        must(w.equals(List.of(1, 99, 4)), "subList can add to the parent");
        must(w.indexOf(4) == 2 && w.indexOf(5) == -1, "indexOf");
        must(Collections.binarySearch(List.of(1, 3, 5), 4) == -3, "binarySearch miss is -(insertion point) - 1");
        Collections.reverse(w);
        must(w.equals(List.of(4, 99, 1)), "reverse");
        int[] x = {1, 2};
        int[] y = x;
        y[0] = 9;
        must(x[0] == 9, "arrays are shared");
        ArrayList<Integer> sized = new ArrayList<>(10);
        sized.ensureCapacity(20);
        sized.trimToSize();
        System.out.println("all checks passed");
    }
}
