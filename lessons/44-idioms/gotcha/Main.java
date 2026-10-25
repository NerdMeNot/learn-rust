public class Main {
    record Config(String name, int port, boolean verbose) {}

    public static void main(String[] args) {
        Config base = new Config("service", 8080, false);
        // A record has no "copy with"; you pass the fields you keep.
        Config test = new Config(base.name(), 9000, base.verbose());
        System.out.println(base);
        System.out.println(test);
    }
}
