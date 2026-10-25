public class Main {
    static class Span implements AutoCloseable {
        private final String name;

        Span(String name) {
            this.name = name;
            System.out.println("enter " + name);
        }

        @Override
        public void close() {
            System.out.println("exit " + name);
        }
    }

    public static void main(String[] args) {
        try (Span span = new Span("request")) {
            System.out.println("listed notes"); // close() runs when the block ends
        }
        new Span("forgotten"); // nothing ever calls close()
        System.out.println("after");
    }
}
