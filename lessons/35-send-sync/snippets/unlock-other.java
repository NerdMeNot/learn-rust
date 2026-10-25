import java.util.concurrent.locks.ReentrantLock;

public class Main {
    public static void main(String[] args) throws InterruptedException {
        ReentrantLock lock = new ReentrantLock();
        lock.lock(); // taken on the main thread
        Thread other = new Thread(() -> {
            try {
                lock.unlock(); // released from a different thread
            } catch (IllegalMonitorStateException e) {
                System.out.println("other thread: " + e.getClass().getSimpleName());
            }
        });
        other.start();
        other.join();
        System.out.println("held by main still: " + lock.isHeldByCurrentThread());
    }
}
