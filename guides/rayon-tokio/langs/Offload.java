import java.util.concurrent.CompletableFuture;
import java.util.stream.LongStream;

public class Offload {
    public static void main(String[] args) throws Exception {
        var total = CompletableFuture.supplyAsync(
            () -> LongStream.range(0, 1_000_000).parallel().map(n -> n % 7).sum());
        System.out.println("common pool: " + total.get());
    }
}
