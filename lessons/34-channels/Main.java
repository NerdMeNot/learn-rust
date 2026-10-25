import java.util.concurrent.LinkedBlockingQueue;

public class Main {
    public static void main(String[] args) throws InterruptedException {
        var queue = new LinkedBlockingQueue<String>();

        var worker = Thread.startVirtualThread(() -> {
            for (int i = 1; i <= 3; i++) {
                queue.add("job " + i);
            }
            queue.add("DONE");   // queues can't be closed, so send a marker
        });

        while (true) {
            var msg = queue.take();
            if (msg.equals("DONE")) break;
            System.out.println("got " + msg);
        }
        worker.join();
    }
}
