import java.util.ArrayDeque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Set;
import java.util.TreeMap;

public class Main {
    public static void main(String[] args) {
        String text = "the cat saw the dog and the cat ran";

        // HashMap: count words. Iteration order isn't fixed, so sort before printing.
        Map<String, Integer> counts = new HashMap<>();
        for (String w : text.split(" ")) {
            counts.merge(w, 1, Integer::sum);
        }
        var top = counts.entrySet().stream()
                .sorted(Map.Entry.<String, Integer>comparingByValue().reversed()
                        .thenComparing(Map.Entry.comparingByKey()))
                .limit(3)
                .toList();
        System.out.println("top: " + top);

        // TreeMap: keys always come out sorted
        Map<String, Integer> byName = new TreeMap<>(Map.of("zoe", 31, "ana", 30, "mia", 28));
        System.out.println("sorted: " + byName);

        // HashSet: unique items, fast "is it in there?"
        Set<String> seen = new HashSet<>(List.of(text.split(" ")));
        System.out.println(seen.size() + " unique words, has dog: " + seen.contains("dog"));

        // ArrayDeque: a queue. Add at the back, take from the front.
        ArrayDeque<String> queue = new ArrayDeque<>(List.of("a", "b"));
        queue.addLast("c");
        String first = queue.pollFirst();
        System.out.println("served " + first + ", waiting " + queue);

        // PriorityQueue gives the smallest first
        PriorityQueue<Integer> heap = new PriorityQueue<>(List.of(5, 1, 8, 3));
        System.out.print("smallest first:");
        while (!heap.isEmpty()) {
            System.out.print(" " + heap.poll());
        }
        System.out.println();
    }
}
