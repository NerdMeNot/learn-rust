import java.nio.charset.StandardCharsets;

public class Main {
    public static void main(String[] args) {
        String owned = "héllo";               // Strings never change
        String view = owned.substring(0, 1);  // a new String

        var s = new StringBuilder("go");      // for text that grows
        s.append(" + rust");

        System.out.println(owned + " " + view + " " + s);
        int bytes = owned.getBytes(StandardCharsets.UTF_8).length;
        System.out.println("bytes=" + bytes + " chars=" + owned.length());
        for (char c : owned.toCharArray()) {
            System.out.print("[" + c + "]");
        }
        System.out.println();
    }
}
