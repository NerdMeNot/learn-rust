import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        var a = new ArrayList<>(List.of(1, 2, 3));
        var b = a;                  // a and b point at the SAME list
        b.set(0, 100);              // change through b...
        System.out.println(a);      // ...and a changed too
        System.out.println(b);

        int x = 5;
        int y = x;                  // primitives like int are copied
        System.out.println(x + " " + y);
    }
}
