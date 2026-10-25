import java.util.Formattable;
import java.util.Formatter;

public class Main {
    record Id(int n) implements Formattable {
        @Override
        public void formatTo(Formatter f, int flags, int width, int precision) {
            boolean left = (flags & java.util.FormattableFlags.LEFT_JUSTIFY) != 0;
            f.format("width=%d left=%b precision=%d%n", width, left, precision);
        }
    }

    public static void main(String[] args) {
        System.out.print(String.format("%-8.3s", new Id(42)));
        System.out.print(String.format("%s", new Id(42)));
    }
}
