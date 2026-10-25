public class Main {
    static String longest(String a, String b) {
        return a.length() >= b.length() ? a : b;
    }

    // join builds a new String at run time: a heap object, not a literal.
    static String join(String... parts) {
        return String.join(" ", parts);
    }

    public static void main(String[] args) {
        String s1 = join("a", "long", "first", "string");
        String result, other;
        {
            String s2 = join("short", "string");
            result = longest(s1, s2);
            String s3 = join("an", "even", "longer", "second", "string");
            other = longest(s1, s3);
        }
        System.out.println("longest: " + result);
        System.out.println("with a longer s3: " + other);
    }
}
