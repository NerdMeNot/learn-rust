import java.util.concurrent.Executors;
import java.util.concurrent.ExecutionException;

public class Main {
    public static void main(String[] args) throws Exception {
        // a plain Thread: the exception ends that thread; join() just returns
        var t = new Thread(() -> {
            throw new IllegalStateException("worker failed");
        }, "worker-1");
        t.start();
        t.join();
        System.out.println("main: join returned normally");

        // a Future from an executor hands the failure back, like Rust's join
        try (var pool = Executors.newSingleThreadExecutor()) {
            var f = pool.submit(() -> {
                throw new IllegalStateException("task failed");
            });
            try {
                f.get();
            } catch (ExecutionException e) {
                System.out.println("main: get() threw ExecutionException, cause: " + e.getCause());
            }
        }
    }
}
