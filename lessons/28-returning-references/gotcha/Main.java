public class Main {
    static String makeGreeting(String name) {
        String s = "hello " + name;
        return s;
    }

    public static void main(String[] args) {
        System.out.println(makeGreeting("rust"));
    }
}
