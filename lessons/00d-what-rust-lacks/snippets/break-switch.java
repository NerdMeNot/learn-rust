public class Main {
    public static void main(String[] args) {
        int n = 0;
        for (int i = 0; i < 5; i++) {
            switch (i) {
                case 2:
                    break;
                default:
                    n++;
            }
        }
        System.out.println("n: " + n);
    }
}
