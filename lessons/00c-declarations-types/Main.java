public class Main {
    static final int LIMIT = 1_000; // Rust: u32. Java has no unsigned integers

    static double half(double x) {
        return x / 2.0;
    }

    public static void main(String[] args) {
        String name = "Ada";
        byte visits = 0; // Rust: u8. Java's byte is signed, the closest it has
        visits++;
        int mask = 0xff;
        float ratio = 2.5f;
        char letter = 'R';
        double total = (double) LIMIT + half(5.0);
        int parsed = Integer.parseInt("42");

        System.out.println("name=" + name + " visits=" + visits);
        System.out.println("mask=" + mask + " ratio=" + ratio + " letter=" + letter);
        System.out.println("limit=" + LIMIT + " total=" + total);
        System.out.println("parsed=" + parsed);
    }
}
