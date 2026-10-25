import java.io.File;
import java.util.ArrayList;
import java.util.concurrent.CountDownLatch;

public class Main {
    static int osThreads() {
        return new File("/proc/self/task").list().length;
    }

    public static void main(String[] args) throws Exception {
        var release = new CountDownLatch(1);
        var started = new CountDownLatch(1000);
        var threads = new ArrayList<Thread>();
        for (int i = 0; i < 1000; i++) {
            threads.add(Thread.ofVirtual().start(() -> {
                started.countDown();
                try {
                    release.await();
                } catch (InterruptedException e) {
                }
            }));
        }
        started.await();
        System.out.println("virtual threads started: 1000, isVirtual: " + threads.get(0).isVirtual());
        System.out.println("OS threads fewer than the virtual threads: " + (osThreads() < 1000));
        release.countDown();
        for (var t : threads) t.join();
    }
}
