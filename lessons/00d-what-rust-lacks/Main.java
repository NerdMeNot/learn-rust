import java.util.List;
import java.util.Optional;

public class Main {
    // What other languages write with ?:, i++, null and switch, written in Java.
    static String size(int n) {
        return switch (n) {
            case 1, 2 -> "small";
            case 3 -> "medium";
            default -> "large";
        };
    }

    static Optional<Integer> firstEven(List<Integer> items) {
        for (int x : items) {
            if (x % 2 == 0) {
                return Optional.of(x);
            }
        }
        return Optional.empty();
    }

    public static void main(String[] args) {
        int total = 0;
        for (int i = 0; i < 4; i++) {
            total += i;
        }
        String label = total > 5 ? "big" : "small";
        System.out.println("total: " + total + " (" + label + ")");
        System.out.println("size: " + size(2) + " " + size(9));
        Optional<Integer> even = firstEven(List.of(3, 5, 8));
        if (even.isPresent()) {
            System.out.println("first even: " + even.get());
        } else {
            System.out.println("no even number");
        }
        System.out.println("none: " + firstEven(List.of(1, 3)).orElse(-1));
        long wide = (long) total * 1_000_000_000;
        System.out.println("wide: " + wide);
    }
}
