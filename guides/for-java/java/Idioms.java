import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.concurrent.CompletableFuture;

sealed interface Shape permits Circle, Square {}
record Circle(double radius) implements Shape {}
record Square() implements Shape {}
record Point(int x, int y) {}

interface Greeter {
    default String hi() { return "hi"; }
}

class English implements Greeter {}

public class Idioms {
    public static void main(String[] args) {
        final var x = 1;
        System.out.println("1: " + x);

        int count = 0;
        count++;
        System.out.println("2: " + count);

        int n = 7;
        String size = n > 5 ? "big" : "small";
        System.out.println("3: " + size);

        String level = switch (n) { case 1, 2 -> "low"; default -> "high"; };
        System.out.println("4: " + level);

        int len = Optional.of("hi").map(String::length).orElse(0);
        System.out.println("5: " + len);

        Map<String, Integer> ages = new HashMap<>();
        ages.put("ana", 31);
        int bo = ages.getOrDefault("bo", 0);
        System.out.println("6: " + bo);

        Map<String, Integer> counts = new HashMap<>();
        counts.merge("a", 1, Integer::sum);
        System.out.println("7: " + counts.get("a"));

        List<Integer> nums = List.of(1, 2, 3, 4);
        List<Integer> squares = nums.stream().filter(m -> m % 2 == 0).map(m -> m * m).toList();
        System.out.println("8: " + squares);

        List<String> names = List.of("ana", "bo");
        StringBuilder seen = new StringBuilder();
        for (String s : names) {
            seen.append(s);
        }
        System.out.println("9: " + seen);

        System.out.println("10: " + String.join(", ", names));

        String word = "héllo";
        System.out.println("11: " + word.length() + " chars");

        String s = "eighty";
        int parsed;
        try { parsed = Integer.parseInt(s); } catch (NumberFormatException e) { parsed = -1; }
        System.out.println("12: " + parsed);

        System.out.println("13: " + new Point(1, 2).equals(new Point(1, 2)));

        for (Shape shape : List.of(new Circle(2.5), new Square())) {
            if (shape instanceof Circle c) {
                System.out.println("14: circle, radius " + c.radius());
            } else {
                System.out.println("14: not a circle");
            }
        }

        System.out.println("15: " + new English().hi());

        List<String> original = List.of("a", "b");
        List<String> copy = new ArrayList<>(original);
        copy.add("c");
        System.out.println("16: " + original.size() + " " + copy.size());

        System.out.println("17: " + CompletableFuture.supplyAsync(() -> 5).join());
        System.out.println("17 pool: " + CompletableFuture.supplyAsync(() -> Thread.currentThread().getName()).join());

        Object lock = new Object();
        int[] box = {0};
        synchronized (lock) { box[0]++; }
        System.out.println("18: " + box[0]);

        System.out.println("19: " + (int) 3.9);

        System.out.println("20: " + "x=" + n);
    }
}
