public class Main {
    static class Counter {
        int count = 0;

        void bump() {
            count++;
        }
    }

    public static void main(String[] args) {
        final var c = new Counter();
        c.bump();
        System.out.println(c.count);
    }
}
