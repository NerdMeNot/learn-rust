public class Main {
    interface Speaker {
        String speak();
    }

    // The standard answer for a type you cannot edit: a wrapper that holds it.
    static class SpeakingString implements Speaker {
        private final String text;

        SpeakingString(String text) {
            this.text = text;
        }

        public String speak() {
            return "the text " + text;
        }
    }

    static <T extends Speaker> String introduce(T s) {
        return s.speak();
    }

    public static void main(String[] args) {
        System.out.println(introduce(new SpeakingString("hi")));
    }
}
