public class Main {
    static long depth(long n) {
        return n == 0 ? 0 : 1 + depth(n - 1);
    }

    public static void main(String[] args) {
        System.out.println("depth(500): " + depth(500));
        try {
            System.out.println("depth(100_000_000): " + depth(100_000_000));
        } catch (StackOverflowError e) {
            System.out.println("caught StackOverflowError, still running");
        }
    }
}
