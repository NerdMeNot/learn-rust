import java.util.stream.LongStream;

public class Sum {
    public static void main(String[] args) {
        long total = LongStream.rangeClosed(1, 1_000_000).parallel().map(n -> n * n).sum();
        System.out.println(total);
    }
}
