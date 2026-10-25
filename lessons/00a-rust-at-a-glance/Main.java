public class Main {
    // Greet someone, then add up 1 to 4.
    static String greet(String name) {
        return "Hello, " + name + "!";
    }

    static int add(int a, int b) {
        return a + b;
    }

    public static void main(String[] args) {
        System.out.println(greet("Ferris"));
        int total = 0;
        for (int n = 1; n <= 4; n++) {
            total = add(total, n);
        }
        System.out.println("total: " + total);
    }
}
