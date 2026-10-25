import java.util.HashMap;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.TreeMap;

public class Main {
    public static void main(String[] args) {
        Map<String, Integer> hash = new HashMap<>();
        Map<String, Integer> linked = new LinkedHashMap<>();
        Map<String, Integer> tree = new TreeMap<>();
        String[] keys = {"one", "two", "three", "four", "five", "six"};
        for (int i = 0; i < keys.length; i++) {
            hash.put(keys[i], i + 1);
            linked.put(keys[i], i + 1);
            tree.put(keys[i], i + 1);
        }
        System.out.println("HashMap:       " + hash.keySet());
        System.out.println("LinkedHashMap: " + linked.keySet());
        System.out.println("TreeMap:       " + tree.keySet());

        hash.put(null, 0);
        System.out.println("HashMap null key: " + hash.get(null));
        try {
            tree.put(null, 0);
        } catch (NullPointerException e) {
            System.out.println("TreeMap null key: NullPointerException");
        }

        // merge is the entry API: add 1, or store 1 the first time
        Map<String, Integer> counts = new HashMap<>();
        for (var word : "a b a".split(" ")) {
            counts.merge(word, 1, Integer::sum);
        }
        System.out.println("counts: " + counts);
    }
}
