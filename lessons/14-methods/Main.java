public class Main {
    static class Counter {
        private int count = 0;

        int get() {
            return count;
        }

        void bump() {
            count++;
        }
    }

    public static void main(String[] args) {
        var c = new Counter();
        c.bump();
        c.bump();
        System.out.println(c.get());
    }
}
