import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.locks.ReentrantLock;

public class Main {
    static int syncCount = 0;
    static int lockCount = 0;
    static final Object gate = new Object();
    static final ReentrantLock lock = new ReentrantLock();

    public static void main(String[] args) throws InterruptedException {
        AtomicInteger atomic = new AtomicInteger();
        ConcurrentHashMap<String, Integer> map = new ConcurrentHashMap<>();
        List<Thread> threads = new ArrayList<>();
        for (int t = 0; t < 8; t++) {
            threads.add(new Thread(() -> {
                for (int i = 0; i < 1000; i++) {
                    synchronized (gate) {
                        syncCount++;
                    }
                    lock.lock();
                    try {
                        lockCount++;
                    } finally {
                        lock.unlock();
                    }
                    atomic.incrementAndGet();
                    map.merge("hits", 1, Integer::sum);
                }
            }));
        }
        threads.forEach(Thread::start);
        for (Thread t : threads) {
            t.join();
        }
        System.out.println("synchronized: " + syncCount);
        System.out.println("ReentrantLock: " + lockCount);
        System.out.println("AtomicInteger: " + atomic.get());
        System.out.println("ConcurrentHashMap.merge: " + map.get("hits"));
    }
}
