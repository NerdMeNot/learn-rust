public class Main {
    static class Noisy implements AutoCloseable {
        final String name;

        Noisy(String name) {
            this.name = name;
        }

        @Override
        public void close() {
            System.out.println("  close " + name);
        }
    }

    public static void main(String[] args) {
        try (Noisy a = new Noisy("a"); Noisy b = new Noisy("b")) {
            System.out.println("  end of block");
        }
    }
}
