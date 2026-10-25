import java.util.Map;
import java.util.TreeSet;

public class Main {
    public static void main(String[] args) {
        // Map.of is an ordinary method; the language has no map literal
        Map<String, Integer> ages = Map.of("Ada", 36, "Linus", 28, "Grace", 45);

        for (String name : new TreeSet<>(ages.keySet())) {
            System.out.println(name + ": " + ages.get(name));
        }
        System.out.println("entries: " + ages.size());
    }
}
