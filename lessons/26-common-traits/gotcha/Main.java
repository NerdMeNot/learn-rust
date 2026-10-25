import java.util.HashMap;

public class Main {
    record Version(int major, int minor) {}

    public static void main(String[] args) {
        HashMap<Version, String> notes = new HashMap<>();
        notes.put(new Version(1, 2), "stable");
        System.out.println("found: " + notes.get(new Version(1, 2)));
    }
}
