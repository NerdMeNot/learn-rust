import java.util.Locale;

public class Main {
    record Id(int n) {
        @Override
        public String toString() {
            return "#" + n;
        }
    }

    public static void main(String[] args) {
        System.out.println(String.format("[%8s] [%-8s]", new Id(42), new Id(42)));
        System.out.println(String.format("%.0f %.0f %.0f", 0.5, 1.5, 2.5));
        System.out.println(String.format(Locale.US, "%,.2f", 1234567.5));
        System.out.println(String.format(Locale.GERMANY, "%,.2f", 1234567.5));
    }
}
