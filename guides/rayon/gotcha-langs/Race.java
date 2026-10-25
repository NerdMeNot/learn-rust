import java.util.stream.LongStream;

public class Race {
    public static void main(String[] args) {
        long[] total = {0}; // shared by every worker thread
        LongStream.rangeClosed(1, 1_000_000).parallel().forEach(n -> total[0] += n);
        System.out.println("total: " + total[0] + " expected: 500000500000");
    }
}
