public class Main {
    record Version(int major, int minor) {}

    public static void main(String[] args) {
        try {
            Integer.parseInt("4x");
            throw new AssertionError("parseInt accepted 4x");
        } catch (NumberFormatException e) {
            // a bad number is an exception
        }
        if (Integer.parseInt("42") != 42) throw new AssertionError();
        if (!new Version(1, 2).equals(new Version(1, 2))) throw new AssertionError();
        if (new Version(1, 2).hashCode() != new Version(1, 2).hashCode()) throw new AssertionError();
        // equals takes any Object: a Version is simply not equal to a String
        if (new Version(1, 2).equals("x")) throw new AssertionError();
        // a record is not Comparable; a Comparator chain is how you order one
        if (Comparable.class.isAssignableFrom(Version.class)) throw new AssertionError();
        var order = java.util.Comparator.comparing((Version v) -> v.major()).thenComparing(Version::minor);
        if (order.compare(new Version(1, 10), new Version(1, 9)) <= 0) throw new AssertionError();
        // Double.equals and Double.compare treat NaN as a value
        if (!Double.valueOf(Double.NaN).equals(Double.NaN)) throw new AssertionError();
        if (Double.compare(Double.NaN, 1.0) != 1 || Double.compare(0.0, -0.0) != 1) throw new AssertionError();
        if (Double.NaN == Double.NaN) throw new AssertionError();
        System.out.println("all checks passed");
    }
}
