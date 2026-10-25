public class Main {
    static void must(boolean ok, String what) {
        if (!ok) throw new RuntimeException(what);
    }

    static int add(int a, int b) {
        return a + b;
    }

    static double add(double a, double b) {
        return a + b;
    }

    static int sum(int... xs) {
        int total = 0;
        for (int x : xs) total += x;
        return total;
    }

    record Pair(int q, int r) {}

    static Pair divmod(int a, int b) {
        return new Pair(a / b, a % b);
    }

    static int parse(String s) throws Exception {
        if (s.isEmpty()) throw new Exception("empty");
        return Integer.parseInt(s);
    }

    static void fill(java.util.List<String> xs) {
        xs.add("x");
    }

    static int bump(int n) {
        n += 1; // a parameter can be reassigned
        return n;
    }

    static int twice(int n) {
        return n * 2;
    }

    public static void main(String[] args) {
        must(later(2) == 4, "methods can be declared after use");
        must(add(1, 2) == 3 && add(1.5, 2.5) == 4.0, "overloads");
        must(sum(1, 2, 3) == 6 && sum() == 0, "varargs");
        must(divmod(7, 2).q() == 3 && divmod(7, 2).r() == 1, "record result");
        boolean threw = false;
        try {
            parse("");
        } catch (Exception e) {
            threw = true;
        }
        must(threw, "throws");
        java.util.List<String> xs = new java.util.ArrayList<>();
        fill(xs);
        must(xs.size() == 1, "the caller sees the callee's change");
        must(bump(1) == 2, "reassign parameter");
        java.util.function.Function<Integer, Integer> f = Main::twice;
        must(f.apply(4) == 8, "method reference");
        System.out.println("all checks passed");
    }

    static int later(int n) {
        return n * 2;
    }
}
