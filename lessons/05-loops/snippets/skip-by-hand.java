public class Main {
    public static void main(String[] args) {
        int passes = 0;
        for (int i = 0; i < 5; i++) {
            if (i == 1) {
                i += 2;
            }
            passes++;
            System.out.println("i: " + i);
        }
        System.out.println("passes: " + passes);
    }
}
