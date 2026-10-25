public class Main {
    public static void main(String[] args) {
        // 1, 2, 3 (the end is not included)
        for (int i = 1; i < 4; i++) {
            System.out.println("for " + i);
        }

        int n = 3;
        while (n > 0) {
            n--;
        }

        // break can't hand back a value, so set a variable first
        int tries = 0;
        int found;
        while (true) {
            tries++;
            if (tries == 3) {
                found = tries * 10;
                break;
            }
        }
        System.out.println("n=" + n + " found=" + found);
    }
}
