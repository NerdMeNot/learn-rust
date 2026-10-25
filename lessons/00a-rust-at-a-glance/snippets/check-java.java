import java.util.List;
import java.util.function.IntUnaryOperator;

public class Main {
    static void must(boolean ok, String what) {
        if (!ok) throw new AssertionError(what);
    }

    static int add(int a, int b) {
        return a + b;
    }

    public static void main(String[] args) {
        int x = 5;
        x = 6;
        final int y = 5;
        must(x == 6 && y == 5, "variables");
        must(add(2, 3) == 5, "method");

        int count = 0;
        for (int i = 0; i < 3; i++) {
            count += i;
        }
        must(count == 3, "counted loop");

        List<String> words = List.of("a", "b");
        String joined = "";
        for (String w : words) {
            joined += w;
        }
        must(joined.equals("ab"), "for each");

        int n = 1;
        String s = switch (n) {
            case 1 -> "one";
            default -> "many";
        };
        must(s.equals("one"), "switch expression");

        IntUnaryOperator inc = v -> v + 1;
        must(inc.applyAsInt(1) == 2, "lambda");

        String name = "Ana";
        int age = 30;
        System.out.printf("%s is %d%n", name, age);

        boolean threw = false;
        try {
            int parsed = Integer.parseInt("x");
            must(parsed == 0, "unreachable");
        } catch (NumberFormatException e) {
            threw = true;
        }
        must(threw, "parseInt throws");
        System.out.println("all java cells compile and hold");
    }
}
