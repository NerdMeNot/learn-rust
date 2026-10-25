import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        // every object is already a shared reference
        List<String> log = new ArrayList<>();
        List<String> other = log;

        log.add("from log");
        other.add("from other");

        System.out.println("log: " + String.join(", ", log));
        System.out.println("same list: " + (log == other));
    }
}
