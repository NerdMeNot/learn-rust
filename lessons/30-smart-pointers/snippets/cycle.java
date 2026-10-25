import java.lang.ref.WeakReference;

public class Main {
    static class Node {
        Node other;
    }

    static WeakReference<Node> makeCycle() {
        Node a = new Node();
        Node b = new Node();
        a.other = b;
        b.other = a;
        return new WeakReference<>(a); // does not keep a alive
    }

    public static void main(String[] args) throws InterruptedException {
        WeakReference<Node> weak = makeCycle();
        for (int i = 0; i < 20 && weak.get() != null; i++) {
            System.gc(); // a request, so ask a few times
            Thread.sleep(50);
        }
        System.out.println("cycle collected: " + (weak.get() == null));
    }
}
