import java.time.Duration;
import java.util.List;

public class Main {
    public static void main(String[] args) throws Exception {
        // each argument is its own String: no shell
        Process echo = new ProcessBuilder("echo", "a  b", "$HOME", "one; echo two").start();
        String out = new String(echo.getInputStream().readAllBytes());
        System.out.println("output " + out.trim() + ", exit " + echo.waitFor());

        // a program that runs and fails: waitFor() returns the code, nothing is thrown
        System.out.println("false exits with " + new ProcessBuilder("false").start().waitFor());

        // a program that cannot start throws
        try {
            new ProcessBuilder(List.of("echo hello")).start();
        } catch (java.io.IOException e) {
            System.out.println("cannot start: " + e.getMessage());
        }

        // a Duration is signed
        System.out.println("1s - 2s = " + Duration.ofSeconds(1).minus(Duration.ofSeconds(2)));

        // the process's own environment is read-only; a child's is edited on the builder
        try {
            System.getenv().put("RBC_MODE", "fast");
        } catch (UnsupportedOperationException e) {
            System.out.println("System.getenv() map: UnsupportedOperationException");
        }
        ProcessBuilder pb = new ProcessBuilder("printenv", "RBC_MODE");
        pb.environment().put("RBC_MODE", "fast");
        System.out.println("child sees " + new String(pb.start().getInputStream().readAllBytes()).trim());
    }
}
