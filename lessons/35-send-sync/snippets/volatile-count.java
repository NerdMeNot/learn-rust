public class Main {
    static int plain = 0;
    static volatile int visible = 0;

    public static void main(String[] args) throws InterruptedException {
        Thread[] threads = new Thread[8];
        for (int i = 0; i < threads.length; i++) {
            threads[i] = new Thread(() -> {
                for (int n = 0; n < 1_000_000; n++) {
                    plain++;
                    visible++; // volatile: every read and write is seen, ++ is still two steps
                }
            });
            threads[i].start();
        }
        for (Thread t : threads) {
            t.join();
        }
        System.out.println("expected 8000000");
        System.out.println("lost updates, plain int:    " + (plain < 8_000_000));
        System.out.println("lost updates, volatile int: " + (visible < 8_000_000));
    }
}
