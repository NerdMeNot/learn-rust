public class Main {
    public static void main(String[] args) {
        int day = 6;
        String kind = switch (day) {
            case 1, 2, 3, 4, 5 -> "weekday";
            case 6, 7 -> "weekend";
        };
        System.out.println(kind);
    }
}
