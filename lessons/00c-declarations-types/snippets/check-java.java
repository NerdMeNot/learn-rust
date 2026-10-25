import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Main {
    static final int MAX = 100;

    static int add(int a, int b) {
        return a + b;
    }

    static void must(boolean ok, String what) {
        if (!ok) throw new AssertionError(what);
    }

    public static void main(String[] args) {
        int count = 5;
        final int fixed = 5;
        var list = new ArrayList<Integer>();
        must(count == fixed && list.isEmpty() && MAX == 100 && add(1, 2) == 3, "declarations");

        double price = 2.9;
        int n = 7;
        must((int) price == 2 && (int) -price == -2 && (double) n == 7.0, "casts");

        long big = 5_000L;
        char c = 'R';
        must(big == 5000 && c == 82 && Character.BYTES == 2, "long char");

        java.util.function.IntUnaryOperator inc = x -> x + 1;
        must(inc.applyAsInt(1) == 2, "lambda");

        int wide = 300;
        double nan = Double.NaN;
        must((byte) wide == 44 && (byte) -1 == -1 && (int) nan == 0, "narrowing casts");
        must((int) 1e20 == Integer.MAX_VALUE && Integer.parseInt("42") == 42, "float cast saturates, parseInt");
        double widened = n; // Java widens int to double on its own
        record Point(int x, int y) {}
        int[] arr = {1, 2, 3};
        String name = "Ada";
        List<Integer> nums = List.of(1, 2, 3);
        must(widened == 7.0 && new Point(1, 2).x() == 1 && arr.length == 3 && name.length() == 3 && nums.size() == 3, "types");

        List<String> empty = Collections.<String>emptyList();
        must(empty.isEmpty(), "explicit type argument");
        must(1_000 == 1000 && 0xFF == 255 && 0b101 == 5 && 017 == 15, "literals");
    }
}
