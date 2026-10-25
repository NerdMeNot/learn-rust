import java.util.HashMap;
import java.util.Map;

public class Main {
    public static void main(String[] args) {
        Map<String, Integer> counts = new HashMap<>();
        counts.put("cat", 0);
        for (String w : "cat cat".split(" ")) {
            counts.put(w, counts.get(w) + 1);
        }
        System.out.println(counts);
    }
}
