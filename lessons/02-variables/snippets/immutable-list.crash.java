import java.util.List;

public class Main {
    public static void main(String[] args) {
        var list = List.of(1); // the list object refuses changes, whatever the variable
        list.add(2);
        System.out.println("list: " + list);
    }
}
