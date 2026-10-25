import java.util.HashMap;
import java.util.Map;

public class Main {
    public static void main(String[] args) {
        Map<Double, String> m = new HashMap<>();
        m.put(Double.NaN, "first");
        m.put(Double.NaN, "second");
        System.out.println("NaN twice: size " + m.size() + ", get(NaN) = " + m.get(Double.NaN));

        Map<Double, String> zeros = new HashMap<>();
        zeros.put(0.0, "zero");
        zeros.put(-0.0, "minus zero");
        System.out.println("0.0 and -0.0: size " + zeros.size());
    }
}
