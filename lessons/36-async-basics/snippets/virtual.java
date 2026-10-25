import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CountDownLatch;

public class Main {
    public static void main(String[] args) throws InterruptedException {
        // supplyAsync starts the work on a pool at once: we wait on a latch, never on the result
        CountDownLatch ran = new CountDownLatch(1);
        CompletableFuture<String> future = CompletableFuture.supplyAsync(() -> {
            ran.countDown();
            return "ferris";
        });
        ran.await();
        System.out.println("body ran before anyone asked for the result: " + (ran.getCount() == 0));
        System.out.println("result: " + future.join());

        // virtual threads: plain blocking calls, no async/await. All 1000 block at the same time.
        CountDownLatch release = new CountDownLatch(1);
        List<Thread> threads = new ArrayList<>();
        for (int i = 0; i < 1000; i++) {
            threads.add(Thread.ofVirtual().start(() -> {
                try {
                    release.await(); // a blocking call
                } catch (InterruptedException e) {
                }
            }));
        }
        while (threads.stream().anyMatch(t -> t.getState() != Thread.State.WAITING)) {
            Thread.sleep(5);
        }
        System.out.println("1000 virtual threads blocked at once: "
                + (1000 > Runtime.getRuntime().availableProcessors()) + " (more than the cores)");
        release.countDown();
        for (Thread t : threads) {
            t.join();
        }
        System.out.println("last one virtual: " + threads.get(999).isVirtual());

        // cancelling is cooperative: interrupt() asks, and the blocked call throws
        Thread sleeper = Thread.ofVirtual().start(() -> {
            try {
                Thread.sleep(Duration.ofMinutes(1));
            } catch (InterruptedException e) {
                System.out.println("sleeper: interrupted, so it stopped sleeping");
            }
        });
        while (sleeper.getState() != Thread.State.TIMED_WAITING) {
            Thread.sleep(5);
        }
        sleeper.interrupt();
        sleeper.join();
    }
}
