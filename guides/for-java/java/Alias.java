import java.util.ArrayList;
import java.util.List;

public class Alias {
    public static void main(String[] args) {
        List<String> a = new ArrayList<>();
        a.add("x");
        List<String> b = a;
        b.add("y");
        System.out.println(a);
        System.out.println(a == b);
    }
}
