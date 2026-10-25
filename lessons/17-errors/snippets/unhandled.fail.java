import java.io.IOException;

public class Main {
    // The signature says this method can fail with an IOException.
    static String read(String path) throws IOException {
        throw new IOException("disk error");
    }

    public static void main(String[] args) {
        System.out.println(read("settings.txt")); // not caught, not declared
    }
}
