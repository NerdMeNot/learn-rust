import java.util.HashMap;
import java.util.Map;

public class Main {
    public static void main(String[] args) {
        Map<String, Integer> stock = new HashMap<>();
        stock.put("apple", 3);
        stock.put("pear", 5);
        for (String name : stock.keySet()) {
            stock.put(name + "2", 1);
        }
        System.out.println("not reached");
    }
}
