import java.util.HashMap;
import java.util.Map;

public class Nulls {
    public static void main(String[] args) {
        Map<String, Integer> ages = new HashMap<>();
        ages.put("ana", 31);
        int ana = ages.get("ana");
        System.out.println("ana: " + ana);
        int bo = ages.get("bo");
        System.out.println("bo: " + bo);
    }
}
