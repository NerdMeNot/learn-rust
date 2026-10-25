public class Main {
    static String makeGreeting(String name) {
        return "hello " + name;
    }

    // returns a new String holding the first word
    static String firstWord(String text) {
        return text.substring(0, text.indexOf(' '));
    }

    public static void main(String[] args) {
        var g = makeGreeting("rust");
        var w = firstWord(g);
        System.out.println(g + " / " + w);
    }
}
