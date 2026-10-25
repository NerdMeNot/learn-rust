import java.util.function.IntUnaryOperator;

public class Main {
    static void log(String msg) {
        System.out.println("log: " + msg);
    }

    public static void main(String[] args) {
        int n = 7;

        String size = n > 5 ? "big" : "small";
        System.out.println("ternary: " + size);

        int w = 3;
        int h = 4;
        int area = w * h;
        System.out.println("block: " + area);

        String name = switch (2) {
            case 1 -> "one";
            case 2 -> "two";
            default -> "many";
        };
        System.out.println("switch: " + name);

        int a = 1;
        int b = 2;
        int tmp = a;
        a = b;
        b = tmp;
        System.out.println("swap: " + a + " " + b);

        log("first");
        int c = 2;
        System.out.println("comma: " + c);

        IntUnaryOperator addOne = x -> {
            int y = x + 1;
            return y;
        };
        System.out.println("closure: " + addOne.applyAsInt(41));
    }
}
