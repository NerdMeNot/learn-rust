import java.time.Duration;

public class Main {
    public static void main(String[] args) throws Exception {
        // args doesn't include the program name
        System.out.println(args.length + " arg(s) after the program name");

        // getenv returns null when the variable isn't set
        String port = System.getenv().getOrDefault("RBC_PORT", "8080");
        System.out.println("port " + port);
        String missing = System.getenv("RBC_MISSING");
        System.out.println(missing != null ? "RBC_MISSING=" + missing : "RBC_MISSING: not set");

        // nanoTime is a monotonic clock, Duration is an amount of time
        long start = System.nanoTime();
        Thread.sleep(20);
        Duration took = Duration.ofNanos(System.nanoTime() - start);
        System.out.println("slept at least 20ms: " + (took.compareTo(Duration.ofMillis(20)) >= 0));

        // run another program and capture what it prints
        Process p = new ProcessBuilder("echo", "hello from echo").start();
        System.out.print(new String(p.getInputStream().readAllBytes()));
        System.out.println("exit ok: " + (p.waitFor() == 0));
    }
}
