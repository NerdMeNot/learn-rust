import java.util.ArrayList;

public class Main {
    static final ArrayList<String> NAMES = new ArrayList<>(); // a field: any initializer

    public static void main(String[] args) {
        final var list = new ArrayList<Integer>();
        list.add(1);
        list.add(2);
        NAMES.add("tracker");
        System.out.println("list: " + list + ", names: " + NAMES);
    }
}
