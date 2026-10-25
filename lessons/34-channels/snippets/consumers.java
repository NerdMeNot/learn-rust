import java.util.*;
import java.util.concurrent.*;

public class Main {
    public static void main(String[] args) throws Exception {
        // Several consumers on one queue: each item goes to exactly one of them.
        var queue = new LinkedBlockingQueue<Integer>();
        for (int i = 0; i < 9; i++) queue.put(i);   // put: send
        var taken = new ConcurrentLinkedQueue<Integer>();
        var consumers = new ArrayList<Thread>();
        for (int c = 0; c < 3; c++) {
            consumers.add(Thread.ofPlatform().start(() -> {
                Integer item;
                while ((item = queue.poll()) != null) taken.add(item);   // poll(): try_recv
            }));
        }
        for (var t : consumers) t.join();
        var sorted = new ArrayList<>(taken);
        Collections.sort(sorted);
        System.out.println("3 consumers took " + sorted);

        // poll with a timeout: recv_timeout
        System.out.println("poll(50 ms) on an empty queue: " + queue.poll(50, TimeUnit.MILLISECONDS));

        // A thread pool is threads taking from one queue.
        var pool = (ThreadPoolExecutor) Executors.newFixedThreadPool(2);
        System.out.println("pool queue: " + pool.getQueue().getClass().getSimpleName());

        // Future.get() per task, in submission order: the first exercise
        var tasks = new ArrayList<Callable<Integer>>();
        for (int i = 3; i >= 0; i--) {
            int n = i;
            tasks.add(() -> { Thread.sleep(n * 20L); return n + 6; });   // the first task is the slowest
        }
        var results = new ArrayList<Integer>();
        for (var f : pool.invokeAll(tasks)) results.add(f.get());
        System.out.println("invokeAll results, in input order: " + results);
        pool.shutdown();
    }
}
