import java.util.concurrent.TimeUnit;

public class Main {
    public static void main(String[] args) throws Exception {
        // waitFor(timeout, unit): false if the child is still running when the time is up
        Process slow = new ProcessBuilder("sleep", "60").start();
        System.out.println("finished within the limit: " + slow.waitFor(200, TimeUnit.MILLISECONDS));
        slow.destroyForcibly();
        System.out.println("after destroyForcibly, waitFor() = " + slow.waitFor());

        // a child ended by a signal looks like an exit code of 128 + the signal number
        Process killed = new ProcessBuilder("sh", "-c", "kill -9 $$").start();
        Process exited = new ProcessBuilder("sh", "-c", "exit 137").start();
        System.out.println("kill -9: " + killed.waitFor() + ", exit 137: " + exited.waitFor());
    }
}
