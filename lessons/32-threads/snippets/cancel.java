public class Main {
    public static void main(String[] args) throws Exception {
        var worker = new Thread(() -> {
            try {
                Thread.sleep(60_000); // a blocking call: interrupt() wakes it
                System.out.println("worker: slept the whole minute");
            } catch (InterruptedException e) {
                System.out.println("worker: interrupted");
            }
        });
        worker.start();
        worker.interrupt();
        worker.join();
    }
}
