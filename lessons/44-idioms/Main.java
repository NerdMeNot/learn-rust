public class Main {
    // The constructor is private, so Port.parse is the only way in.
    static final class Port {
        private final int number;

        private Port(int number) {
            this.number = number;
        }

        static Port parse(String text) {
            long n;
            try {
                n = Long.parseLong(text);
            } catch (NumberFormatException e) {
                throw new IllegalArgumentException("not a number");
            }
            if (n < 1 || n > 65535) {
                throw new IllegalArgumentException(n + " is outside 1-65535");
            }
            return new Port((int) n);
        }

        int number() {
            return number;
        }
    }

    static void listen(Port port) {
        System.out.println("listening on " + port.number());
    }

    public static void main(String[] args) {
        for (String text : new String[] {"8080", "0", "http"}) {
            try {
                listen(Port.parse(text));
            } catch (IllegalArgumentException e) {
                System.out.println("\"" + text + "\": " + e.getMessage());
            }
        }
    }
}
