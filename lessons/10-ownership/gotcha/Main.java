import java.util.List;

public class Main {
    public static void main(String[] args) {
        var a = List.of(1, 2, 3);
        var b = a;
        System.out.println(a + " " + b);
    }
}
