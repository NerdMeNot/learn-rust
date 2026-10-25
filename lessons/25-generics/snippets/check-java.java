import java.io.Serializable;
import java.util.Collections;
import java.util.List;
import java.util.function.Function;

// Backs the Java cells of the table in lesson.mdx. Never shown.
public class Main {
    static void must(boolean ok, String what) {
        if (!ok) throw new AssertionError(what);
    }

    static <T extends Comparable<T> & Serializable> T bigger(T a, T b) {
        return a.compareTo(b) > 0 ? a : b;
    }

    static double total(List<? extends Number> xs) {
        double sum = 0;
        for (Number n : xs) sum += n.doubleValue();
        return sum;
    }

    // An unbounded T still has toString, equals and hashCode.
    static <T> String describe(T x) {
        return x.toString() + " " + x.equals(x) + " " + x.hashCode();
    }

    record Pair<T>(T first, T second) {
        <R> Pair<R> map(Function<? super T, ? extends R> f) {
            return new Pair<>(f.apply(first), f.apply(second));
        }
    }

    public static void main(String[] args) {
        must(describe(7).equals("7 true 7"), "unbounded T");
        must(new Pair<>(1, 2).map(n -> "#" + n).first().equals("#1"), "generic method");
        must(Collections.<String>emptyList().isEmpty(), "explicit type argument");
        must(bigger("a", "b").equals("b"), "two bounds");
        List<Integer> ints = List.of(1, 2);
        must(total(ints) == 3.0, "wildcard");
        System.out.println("ok");
    }
}
