import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.IntUnaryOperator;

public class Main {
    public static void main(String[] args) {
        int factor = 3;
        IntUnaryOperator times = x -> x * factor;   // reads factor
        System.out.println(times.applyAsInt(5));

        // a lambda can't reassign a local, so keep the count in an object
        var count = new AtomicInteger();
        Runnable inc = count::incrementAndGet;
        inc.run();
        inc.run();
        System.out.println("count=" + count);
    }
}
