import java.util.ArrayList;
import java.util.List;

public class Main {
    record Job(String name, List<String> log) {}

    static void run(Job job) {
        job.log().add(job.name());
    }

    public static void main(String[] args) throws Exception {
        List<String> log = new ArrayList<>();
        Job job = new Job("resize", log);
        Thread t = new Thread(() -> run(job));
        t.start();
        t.join();
        System.out.println("log: [\"" + String.join("\", \"", log) + "\"]");
    }
}
