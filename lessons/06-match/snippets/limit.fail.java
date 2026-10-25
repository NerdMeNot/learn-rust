public class Main {
    public static void main(String[] args) {
        int limit = 10;
        int n = 10;
        String text = switch (n) {
            case limit -> "at the limit";
            default -> "under";
        };
        System.out.println(text);
    }
}
