import java.util.List;

public class Main {
    public static void main(String[] args) {
        var names = List.of("ana", "bo");
        for (var n : names) {
            System.out.println("hi " + n);
        }
        System.out.println(names.size() + " names");
    }
}
