public class Main {
    static class Parser {
        private String rest;

        Parser(String input) {
            rest = input;
        }

        String nextWord() {
            String trimmed = rest.stripLeading();
            if (trimmed.isEmpty()) {
                return null;
            }
            int space = trimmed.indexOf(' ');
            rest = space < 0 ? "" : trimmed.substring(space + 1);
            return space < 0 ? trimmed : trimmed.substring(0, space);
        }
    }

    public static void main(String[] args) {
        Parser parser = new Parser("lifetimes are regions");
        String a = parser.nextWord();
        String b = parser.nextWord(); // a is still usable: no annotation needed
        System.out.println("a: " + a);
        System.out.println("b: " + b);
        String again = new Parser("lifetimes are regions").nextWord();
        System.out.println("same object as a: " + (a == again));
    }
}
