import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.function.Function;

public class Main {
    static <T extends Comparable<T>> T largest(List<T> list) {
        T best = list.get(0);
        for (T item : list) {
            if (item.compareTo(best) > 0) {
                best = item;
            }
        }
        return best;
    }

    static int apply(Function<Integer, Integer> f, int x) {
        return f.apply(x);
    }

    static Optional<Integer> find(Map<String, Integer> m, String k) {
        return Optional.ofNullable(m.get(k));
    }

    static String read(String path) throws IOException {
        return Files.readString(Path.of(path));
    }

    interface Describe {
        String describe();
    }

    static String show(Describe d) {
        return d.describe();
    }

    static <T extends Number & Comparable<T>> T bigger(T a, T b) {
        return a.compareTo(b) > 0 ? a : b;
    }

    static String twice(String path) throws IOException, IllegalStateException {
        return read(path) + read(path);
    }

    // `Comparable<? super T>` lets a subclass whose parent implements Comparable qualify
    static <T extends Comparable<? super T>> T largestWild(List<T> list) {
        T best = list.get(0);
        for (T item : list) {
            if (item.compareTo(best) > 0) {
                best = item;
            }
        }
        return best;
    }

    static class Base implements Comparable<Base> {
        final int n;

        Base(int n) {
            this.n = n;
        }

        public int compareTo(Base o) {
            return Integer.compare(n, o.n);
        }
    }

    static class Derived extends Base {
        Derived(int n) {
            super(n);
        }
    }

    static class Log {
        private final List<String> lines = new ArrayList<>();

        void add(String line) {
            lines.add(line);
        }

        int count() {
            return lines.size();
        }
    }

    public static void main(String[] args) {
        if (!largest(List.of(3, 9, 4)).equals(9)) throw new AssertionError("largest");
        if (apply(x -> x + 1, 1) != 2) throw new AssertionError("apply");
        if (find(new HashMap<>(), "a").isPresent()) throw new AssertionError("find");
        try {
            read("/nope");
            throw new AssertionError("read");
        } catch (IOException e) {
            // the signature said so: `throws IOException`
        }
        if (!show(() -> "sq").equals("sq")) throw new AssertionError("show");
        if (bigger(1, 2) != 2) throw new AssertionError("bigger");
        if (largestWild(List.of(new Derived(1), new Derived(3))).n != 3) throw new AssertionError("wild");
        Log log = new Log();
        log.add("hello");
        if (log.count() != 1) throw new AssertionError("Log");
        System.out.println("int bits: " + Integer.SIZE);
        System.out.println("java rows ok");
    }
}
