import java.util.Optional;

public class Main {
    public static void main(String[] args) {
        // Optional is a class like any other, so a variable of that type can be null.
        Optional<String> nickname = null;
        System.out.println("compiled and started");
        System.out.println(nickname.isPresent());
    }
}
