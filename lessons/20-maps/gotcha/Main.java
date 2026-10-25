import java.util.HashMap;
import java.util.Map;

public class Main {
    public static void main(String[] args) {
        Map<String, Integer> ages = new HashMap<>();
        ages.put("ana", 30);
        int age = ages.get("ana");
        System.out.println(age);
    }
}
