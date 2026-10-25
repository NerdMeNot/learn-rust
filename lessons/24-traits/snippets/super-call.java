public class Main {
    interface Speaker {
        String speak();

        default String shout() {
            return speak().toUpperCase();
        }
    }

    static class Parrot implements Speaker {
        public String speak() {
            return "hello";
        }

        @Override
        public String shout() {
            return Speaker.super.shout() + "!!"; // the default this method replaces
        }
    }

    public static void main(String[] args) {
        System.out.println(new Parrot().shout());
    }
}
