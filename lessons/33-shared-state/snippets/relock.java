public class Main {
    static final Object lock = new Object();

    static void inner() {
        synchronized (lock) {
            System.out.println("inner: same thread, lock already held, not blocked");
        }
    }

    public static void main(String[] args) {
        synchronized (lock) {
            inner();
        }
    }
}
