public class Main {
    public static void main(String[] args) throws InterruptedException {
        System.out.println("start");
        // Java has no async/await. A virtual thread that is built but not started
        // is the nearest thing to "created, not running".
        Thread task = Thread.ofVirtual().unstarted(() -> System.out.println("task body runs"));
        System.out.println("task created");
        task.start();
        task.join();
        System.out.println("end");
    }
}
