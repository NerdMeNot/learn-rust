import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;

public class Main {
    // A record writes equals, hashCode and toString from its components.
    // Ordering is not automatic: Comparable is the one to implement.
    record Version(int major, int minor) implements Comparable<Version> {
        @Override
        public int compareTo(Version other) {
            return major != other.major ? Integer.compare(major, other.major) : Integer.compare(minor, other.minor);
        }

        // toString is the user-facing text (a record's own would be "Version[major=1, minor=10]").
        @Override
        public String toString() {
            return "v" + major + "." + minor;
        }
    }

    public static void main(String[] args) {
        Version a = new Version(1, 10);
        Version b = new Version(1, 9);
        System.out.println("display: " + a);
        System.out.println("a == b: " + a.equals(b));
        System.out.println("a > b: " + (a.compareTo(b) > 0));

        List<Version> all = new ArrayList<>(List.of(a, b, new Version(0, 0)));
        all.sort(null);
        List<String> names = new ArrayList<>();
        for (Version v : all) names.add(v.toString());
        System.out.println("sorted: " + String.join(" ", names));

        HashSet<Version> seen = new HashSet<>();
        for (Version v : List.of(a, b, new Version(0, 0), a)) seen.add(v);
        System.out.println("distinct: " + seen.size());
    }
}
