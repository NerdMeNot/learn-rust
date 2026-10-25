import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) throws InterruptedException {
        List<Integer> results = new ArrayList<>();
        List<Thread> threads = new ArrayList<>();

        for (int i = 0; i < 3; i++) {
            int n = i;
            threads.add(Thread.startVirtualThread(() -> {
                synchronized (results) {   // lock, add, unlock
                    results.add(n * 10);
                }
            }));
        }
        for (var t : threads) {
            t.join();
        }

        results.sort(null);
        System.out.println(results);
    }
}
