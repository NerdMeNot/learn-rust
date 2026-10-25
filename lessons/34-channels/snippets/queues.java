import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.LinkedBlockingQueue;
import java.util.concurrent.SynchronousQueue;

public class Main {
    public static void main(String[] args) {
        // unbounded, like channel(): the default capacity is Integer.MAX_VALUE
        var unbounded = new LinkedBlockingQueue<Integer>();
        System.out.println("LinkedBlockingQueue room: " + unbounded.remainingCapacity());

        // bounded, like sync_channel(2): offer is try_send
        var bounded = new ArrayBlockingQueue<Integer>(2);
        System.out.println("offer 1: " + bounded.offer(1));
        System.out.println("offer 2: " + bounded.offer(2));
        System.out.println("offer 3: " + bounded.offer(3));

        // no buffer at all, like sync_channel(0): a hand-off needs a taker
        var handoff = new SynchronousQueue<Integer>();
        System.out.println("offer with no taker: " + handoff.offer(1));
    }
}
