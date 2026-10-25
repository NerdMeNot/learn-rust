public class Main {
    // Number has no `+`: Java's operators work only on the primitive types.
    static <T extends Number> T add(T a, T b) {
        return a + b;
    }

    public static void main(String[] args) {
        System.out.println(add(2, 3));
        System.out.println(add(1.5, 2.25));
    }
}
