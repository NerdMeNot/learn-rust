public class Main {
    static int parseAge(String s) {
        return Integer.parseInt(s); // unchecked: no throws clause needed
    }

    static int load() {
        return parseAge("abc");
    }

    public static void main(String[] args) {
        System.out.println(load());
    }
}
