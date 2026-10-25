import java.util.HashMap;

public class Main {
    // A plain class: equals and hashCode are the inherited identity versions.
    static class Plain {
        final int major, minor;

        Plain(int major, int minor) {
            this.major = major;
            this.minor = minor;
        }
    }

    // equals written, hashCode forgotten: the usual mistake.
    static class HalfDone extends Plain {
        HalfDone(int major, int minor) {
            super(major, minor);
        }

        @Override
        public boolean equals(Object o) {
            return o instanceof HalfDone h && h.major == major && h.minor == minor;
        }
    }

    record Version(int major, int minor) {}

    public static void main(String[] args) {
        var plain = new HashMap<Plain, String>();
        plain.put(new Plain(1, 2), "stable");
        System.out.println("plain class: " + plain.get(new Plain(1, 2)));

        var half = new HashMap<HalfDone, String>();
        half.put(new HalfDone(1, 2), "stable");
        System.out.println("equals only: " + half.get(new HalfDone(1, 2)));
        System.out.println("equals says: " + new HalfDone(1, 2).equals(new HalfDone(1, 2)));

        var rec = new HashMap<Version, String>();
        rec.put(new Version(1, 2), "stable");
        System.out.println("record: " + rec.get(new Version(1, 2)));
    }
}
