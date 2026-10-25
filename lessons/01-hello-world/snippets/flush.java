import java.util.List;

public class Main {
    public static void main(String[] args) throws Exception {
        if (args.length > 0) {
            System.out.print("a partial line");
            Runtime.getRuntime().halt(3); // an exit that skips normal cleanup
        }
        String java = ProcessHandle.current().info().command().orElseThrow();
        String classpath = System.getProperty("java.class.path");
        Process child = new ProcessBuilder(List.of(java, "-cp", classpath, "Main", "child")).start();
        String out = new String(child.getInputStream().readAllBytes());
        child.waitFor();
        System.out.println("stdout is a pipe: " + out.length() + " characters: \"" + out + "\"");
    }
}
