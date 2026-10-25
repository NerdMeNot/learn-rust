import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

public class Main {
    static String read(String path) throws IOException {
        return Files.readString(Path.of(path));
    }

    public static void main(String[] args) {
        System.out.println(read("/nope"));
    }
}
