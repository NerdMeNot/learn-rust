public class Main {
    // Nothing to annotate: the JVM keeps a String alive while any reference to it exists.
    static String longest(String a, String b) {
        return a.length() >= b.length() ? a : b;
    }

    record Excerpt(String part) {}

    public static void main(String[] args) {
        String novel = "Call me Ishmael. Some years ago...";
        String first = novel.substring(0, novel.indexOf('.'));
        Excerpt excerpt = new Excerpt(first);
        String l = longest(excerpt.part(), "short");
        System.out.println("longest: " + l);
        System.out.println("excerpt length: " + excerpt.part().length());
    }
}
