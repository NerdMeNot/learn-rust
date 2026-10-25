import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.TimeoutException;

public class Main {
    static String fetch(String name, int ms) {
        try {
            Thread.sleep(ms);
        } catch (InterruptedException e) {
            throw new RuntimeException(e);
        }
        return name + " done after " + ms + "ms";
    }

    public static void main(String[] args) throws Exception {
        var users = CompletableFuture.supplyAsync(() -> fetch("users", 30));
        var orders = CompletableFuture.supplyAsync(() -> fetch("orders", 10));
        System.out.println(users.get());
        System.out.println(orders.get());

        var left = CompletableFuture.supplyAsync(() -> fetch("left", 20));
        var right = CompletableFuture.supplyAsync(() -> fetch("right", 20));
        System.out.println(left.get() + " / " + right.get());

        try {
            var slow = CompletableFuture.supplyAsync(() -> fetch("slow", 500))
                    .orTimeout(50, TimeUnit.MILLISECONDS);
            System.out.println(slow.get());
        } catch (ExecutionException e) {
            if (e.getCause() instanceof TimeoutException) {
                System.out.println("slow call timed out");
            } else {
                throw e;
            }
        }
    }
}
