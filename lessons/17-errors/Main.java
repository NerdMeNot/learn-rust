import java.util.List;

public class Main {
    static int parseAge(String s) {
        return Integer.parseInt(s);   // throws NumberFormatException on bad input
    }

    public static void main(String[] args) {
        for (var s : List.of("42", "abc")) {
            try {
                System.out.println("age " + parseAge(s));
            } catch (NumberFormatException e) {
                System.out.println("error: " + e.getMessage());
            }
        }
    }
}
