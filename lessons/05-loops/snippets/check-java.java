public class Main {
    static void must(boolean ok, String what) {
        if (!ok) {
            throw new IllegalStateException(what);
        }
    }

    public static void main(String[] args) {
        int sum = 0;
        for (int x : new int[] {1, 2, 3}) {
            sum += x;
        }
        must(sum == 6, "enhanced for");

        int hits = 0;
        outer:
        for (int a = 0; a < 3; a++) {
            for (int b = 0; b < 3; b++) {
                if (b == 1) {
                    continue outer;
                }
                if (a == 2) {
                    break outer;
                }
                hits++;
            }
        }
        must(hits == 2, "labels");

        int n = 0;
        do {
            n += 5;
        } while (n < 3);
        must(n == 5, "do-while runs once");

        // break inside a switch leaves the switch only
        int printed = 0;
        for (int v = 1; v <= 5; v++) {
            switch (v) {
                case 3:
                    break;
                default:
                    printed++;
            }
        }
        must(printed == 4, "break in switch");

        // IntStream.range is start and end (end excluded); rangeClosed includes the end
        must(java.util.stream.IntStream.range(1, 4).sum() == 6, "range");
        must(java.util.stream.IntStream.rangeClosed(1, 3).sum() == 6, "rangeClosed");

        // continue in a for runs the step; in a do-while it goes to the test
        int steps = 0;
        for (int i = 0; i < 3; i++) {
            if (i == 1) {
                continue;
            }
            steps++;
        }
        must(steps == 2, "continue runs i++");
        int k = 0;
        int passes = 0;
        do {
            k++;
            passes++;
            if (k >= 3) {
                continue;
            }
        } while (k < 3);
        must(k == 3 && passes == 3, "continue in do-while");
        System.out.println("ok");
    }
}
