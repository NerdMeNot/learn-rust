public class Main {
    // toString is Java's Display
    record Money(long cents) {
        @Override
        public String toString() {
            return String.format("$%d.%02d", cents / 100, cents % 100);
        }
    }

    // a record writes toString for you, like derive(Debug)
    record Item(String name, int qty) {}

    public static void main(String[] args) {
        int n = 42;
        System.out.printf("[%6d] [%-6d] [%06d]%n", n, n, n); // no centering flag
        System.out.printf("%.2f %e %#x 0b%s%n", 3.14159, 1234.5, n, Integer.toBinaryString(n));

        Money price = new Money(1999);
        System.out.println("total: " + price);

        String label = price + " each";
        System.out.println(label + " (" + label.length() + " chars)");

        Item item = new Item("pen", 3);
        System.out.println(item.qty() + " x " + item.name());
        System.out.println(item);
    }
}
