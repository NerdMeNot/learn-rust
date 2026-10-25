public class Main {
    public static void main(String[] args) {
        int a = 7;            // 32-bit signed integer
        long b = 10;          // 64-bit signed integer
        double price = 2.5;   // 64-bit float
        boolean ok = true;

        long total = a + b;       // Java widens int to long for you
        double cost = a * price;  // and int to double
        System.out.println(total + " " + cost + " " + ok);
    }
}
