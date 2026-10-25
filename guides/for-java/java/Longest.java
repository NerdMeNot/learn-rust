public class Longest {
    static String longest(String a, String b) {
        return a.length() >= b.length() ? a : b;
    }

    public static void main(String[] args) {
        String result;
        {
            String inner = new String("a longer string");
            result = longest("short", inner);
        }
        System.out.println(result);
    }
}
