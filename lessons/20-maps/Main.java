import java.util.HashMap;
import java.util.Map;

public class Main {
    public static void main(String[] args) {
        Map<String, Integer> ages = new HashMap<>();
        ages.put("ana", 30);
        ages.put("bo", 25);

        // get returns null when the key is missing
        Integer ana = ages.get("ana");
        System.out.println(ana != null ? "ana is " + ana : "no ana");

        int zed = ages.getOrDefault("zed", 0);
        System.out.println("zed is " + zed);

        // count words: merge adds 1, or inserts 1 the first time
        Map<String, Integer> counts = new HashMap<>();
        for (var word : "a b a c a".split(" ")) {
            counts.merge(word, 1, Integer::sum);
        }
        System.out.println("a appears " + counts.get("a") + " times");
    }
}
