import java.util.Arrays;
import java.util.Locale;

public class Main {
    public static void main(String[] args) {
        System.out.println("\"a,b,,\".split(\",\"):     " + Arrays.toString("a,b,,".split(",")));
        System.out.println("\"a,b,,\".split(\",\", -1): " + Arrays.toString("a,b,,".split(",", -1)));
        System.out.println("\"\".split(\",\").length:    " + "".split(",").length);
        System.out.println("\"a.b\".split(\".\").length:  " + "a.b".split(".").length);
        System.out.println("\"a|b\".split(\"|\"):       " + Arrays.toString("a|b".split("|")));
        System.out.println("\"a.b\".split(\"\\\\.\"):     " + Arrays.toString("a.b".split("\\.")));

        for (String text : new String[] {"42", "+42", " 42", "1_000"}) {
            try {
                System.out.println("parseInt(\"" + text + "\"): " + Integer.parseInt(text));
            } catch (NumberFormatException e) {
                System.out.println("parseInt(\"" + text + "\"): " + e.getMessage());
            }
        }

        System.out.println("\"a,b,c\".split(\",\", 2):   " + Arrays.toString("a,b,c".split(",", 2)));
        System.out.println("\"a,b,,\".split(\",\", 5):  " + Arrays.toString("a,b,,".split(",", 5)));
        System.out.println("\" a  b\".split(\"\\\\s+\"):   " + Arrays.toString(" a  b".split("\\s+")));
        System.out.println("\"a.b.c\".replace(\".\", \"-\"):    " + "a.b.c".replace(".", "-"));
        System.out.println("\"a.b.c\".replaceAll(\".\", \"-\"): " + "a.b.c".replaceAll(".", "-"));
        System.out.println("\"abc123\".matches(\"\\\\d+\"):  " + "abc123".matches("\\d+"));

        // Whitespace: length after strip() and after trim() of NBSP, U+0001 and U+3000 on both sides.
        for (String pad : new String[] {"\u00a0", "\u0001", "\u3000"}) {
            String text = pad + "hi" + pad;
            System.out.println(String.format("U+%04X around hi: strip %d, trim %d", (int) pad.charAt(0), text.strip().length(), text.trim().length()));
        }
        System.out.println("equalsIgnoreCase(\"É\", \"é\"): " + "É".equalsIgnoreCase("é"));
        System.out.println("parseInt(\"٣\"): " + Integer.parseInt("٣"));
        System.out.println("\"a\\rb\\r\\nc\\n\".lines(): " + "a\rb\r\nc\n".lines().toList());

        System.out.println("\"straße\".toUpperCase(): " + "straße".toUpperCase());
        Locale.setDefault(Locale.forLanguageTag("tr"));
        System.out.println("\"TITLE\".toLowerCase() under a Turkish default: " + "TITLE".toLowerCase());
        System.out.println("\"TITLE\".toLowerCase(Locale.ROOT):             " + "TITLE".toLowerCase(Locale.ROOT));
    }
}
