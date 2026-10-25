public class Main {
    public static void main(String[] args) {
        int n = 7;
        if (n % 2 == 0) {
            System.out.println(n + " is even");
        } else {
            System.out.println(n + " is odd");
        }

        // the ternary operator is Java's if-as-a-value
        String size = n > 5 ? "big" : "small";
        System.out.println(size);
    }
}
