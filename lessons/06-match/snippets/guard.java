public class Main {
    sealed interface Shape permits Circle, Rect {}
    record Circle(int r) implements Shape {}
    record Rect(int w, int h) implements Shape {}

    static String classify(Object o) {
        return switch (o) {
            case Integer i when i > 100 -> "big " + i;
            case Integer i when i >= 1 && i <= 5 -> "few " + i;
            case Integer i -> "other number " + i;
            case String s -> "text of " + s.length() + " chars";
            default -> "something else";
        };
    }

    static String size(int n) {
        return switch (n) {
            case 1, 2, 3 -> "tiny";
            case 4, 5 -> "small";
            default -> "bigger";
        };
    }

    static String describe(Shape s) {
        return switch (s) {
            case Circle c when c.r() > 10 -> "big circle";
            case Circle(int r) -> "circle " + r;
            case Rect(int w, int h) when w == h -> "square " + w;
            case Rect(int w, int h) -> "rect " + w + "x" + h;
        };
    }

    public static void main(String[] args) {
        System.out.println(classify(500));
        System.out.println(classify(3));
        System.out.println(classify(-2));
        System.out.println(classify("hello"));
        System.out.println(classify(2.5));
        System.out.println(size(2) + " " + size(5) + " " + size(9));
        System.out.println(describe(new Circle(20)));
        System.out.println(describe(new Circle(2)));
        System.out.println(describe(new Rect(3, 3)));
        System.out.println(describe(new Rect(2, 5)));
    }
}
