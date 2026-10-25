public class Main {
    public static void main(String[] args) {
        String name = switch (2) {
            case 1 -> "one";
            case 2 -> {
                int w = 3;
                int h = 4;
                yield "area " + (w * h);
            }
            default -> "many";
        };
        System.out.println("yield: " + name);
    }
}
