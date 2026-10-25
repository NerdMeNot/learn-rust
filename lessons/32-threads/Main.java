import java.util.List;
import java.util.concurrent.Executors;

public class Main {
    public static void main(String[] args) throws Exception {
        var names = List.of("a", "b", "c");

        try (var pool = Executors.newSingleThreadExecutor()) {
            // a Callable can return a value, unlike a plain Thread
            var future = pool.submit(() -> {
                for (var n : names) {
                    System.out.println("worker sees " + n);
                }
                return names.size();
            });

            int count = future.get();   // wait for the worker
            System.out.println("worker counted " + count);
        }
    }
}
