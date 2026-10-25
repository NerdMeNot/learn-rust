import java.util.concurrent.CountDownLatch;

public class Main {
    public static void main(String[] args) throws Exception {
        var go = new CountDownLatch(1);
        var never = new CountDownLatch(1);

        var worker = new Thread(() -> {
            try {
                go.await();
            } catch (InterruptedException e) {
            }
            System.out.println("worker: finished after main returned");
        });
        worker.start(); // non-daemon by default

        var daemon = new Thread(() -> {
            try {
                never.await(); // blocks forever
            } catch (InterruptedException e) {
            }
        });
        daemon.setDaemon(true);
        daemon.start();

        System.out.println("main: returning");
        go.countDown();
    }
}
