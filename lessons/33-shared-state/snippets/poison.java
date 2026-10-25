import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) throws InterruptedException {
        List<Integer> data = new ArrayList<>(List.of(1, 2, 3));
        Object lock = new Object();

        Thread worker = new Thread(() -> {
            try {
                synchronized (lock) {
                    data.add(4);
                    throw new IllegalStateException("worker failed halfway");
                }
            } catch (IllegalStateException e) {
                System.out.println("worker died: " + e.getMessage());
            }
        });
        worker.start();
        worker.join();

        synchronized (lock) { // not poisoned: leaving the block released the lock
            System.out.println("main got the lock; data: " + data);
        }
    }
}
