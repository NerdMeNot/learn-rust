public class Parse {
    static int port(String text) {
        return Integer.parseInt(text);
    }

    public static void main(String[] args) {
        System.out.println(port("8080"));
        System.out.println(port("eighty"));
    }
}
