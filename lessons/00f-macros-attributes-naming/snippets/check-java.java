public class Main {
    static boolean bad = false;

    static void same(String got, String want) {
        if (!got.equals(want)) {
            System.out.println("got [" + got + "], want [" + want + "]");
            bad = true;
        }
    }

    public static void main(String[] args) {
        same(String.format("%5d", 42), "   42");
        same(String.format("%-5d", 42), "42   ");
        same(String.format("%05d", 42), "00042");
        same(String.format("%+d", 42), "+42");
        same(String.format("%.2f", 3.14159), "3.14");
        same(String.format("%8.3f", 3.14159), "   3.142");
        same(String.format("%x", 255), "ff");
        same(String.format("%#x", 255), "0xff");
        same(String.format("%10s", "Ada"), "       Ada");
        same(String.format("%-10s", "Ada"), "Ada       ");
        same(String.format("%.3s", "Lovelace"), "Lov");
        same(String.format("%d %s", 42, "Ada"), "42 Ada");
        if (bad) throw new AssertionError("a Java format row is wrong");
        System.out.println("ok");
    }
}
