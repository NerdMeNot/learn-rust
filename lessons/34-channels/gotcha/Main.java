import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.LinkedBlockingQueue;

public class Main {
    public static void main(String[] args) throws InterruptedException {
        var queue = new LinkedBlockingQueue<List<Integer>>();
        Thread.startVirtualThread(() -> {
            var msg = new ArrayList<>(List.of(1, 2));
            queue.add(msg);
            msg.add(3);   // sender keeps changing what it already sent
        });
        var got = queue.take();
        Thread.sleep(100);
        System.out.println(got);
    }
}
