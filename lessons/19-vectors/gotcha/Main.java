import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        List<String> names = new ArrayList<>(List.of("ana", "bo"));
        String first = names.get(0);
        System.out.println(first + " of " + names.size());
    }
}
