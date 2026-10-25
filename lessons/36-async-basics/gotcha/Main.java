import java.util.concurrent.CompletableFuture;

public class Main {
    static CompletableFuture<Integer> loadCount() {
        return CompletableFuture.supplyAsync(() -> 3);
    }

    static int report() {
        int n = loadCount();
        return n + 1;
    }

    public static void main(String[] args) {
        System.out.println("count: " + report());
    }
}
