public class Main {
    record MinMax(int lo, int hi) {}

    static MinMax minMax(int a, int b) {
        return new MinMax(Math.min(a, b), Math.max(a, b));
    }

    public static void main(String[] args) {
        MinMax r = minMax(9, 4);
        // A record pattern unpacks in instanceof and switch, not in a plain declaration.
        if (r instanceof MinMax(int lo, int hi)) {
            System.out.println("lo: " + lo + ", hi: " + hi);
        }
        System.out.println("equals: " + r.equals(new MinMax(4, 9)));
        System.out.println("class: " + r.getClass().getName());
        int a = 1;
        int b = 2;
        int tmp = a;
        a = b;
        b = tmp;
        System.out.println("swap: " + a + " " + b);
    }
}
