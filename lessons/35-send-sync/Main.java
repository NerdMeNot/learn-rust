import java.util.List;
import java.util.concurrent.Executors;
import java.util.stream.Collectors;

public class Main {
    public static void main(String[] args) throws Exception {
        var words = List.of("alpha", "beta");

        try (var pool = Executors.newFixedThreadPool(2)) {
            // hand the list to a pool thread and get it back as the result
            var back = pool.submit(() -> words).get();
            var quoted = back.stream().map(w -> "\"" + w + "\"").collect(Collectors.joining(", ", "[", "]"));
            System.out.println("moved there and back: " + quoted);

            // two threads read the same list at once; nothing checks that this is safe
            var a = pool.submit(() -> words.get(0).length());
            var b = pool.submit(() -> words.get(1).length());
            System.out.println("lengths read in parallel: " + a.get() + " and " + b.get());
        }
    }
}
