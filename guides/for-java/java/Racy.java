import java.util.ArrayList;
import java.util.List;

public class Racy {
    public static void main(String[] args) throws InterruptedException {
        List<Integer> results = new ArrayList<>();
        List<Thread> threads = new ArrayList<>();
        for (int i = 0; i < 4; i++) {
            int n = i;
            threads.add(new Thread(() -> {
                for (int j = 0; j < 100_000; j++) {
                    results.add(n * 10);
                }
            }));
        }
        threads.forEach(Thread::start);
        for (var t : threads) {
            t.join();
        }
        System.out.println(results.size() + " items (expected 400000)");
    }
}
