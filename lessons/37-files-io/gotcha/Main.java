import java.io.FileInputStream;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

public class Main {
    public static void main(String[] args) throws IOException {
        Path path = Path.of(System.getProperty("java.io.tmpdir"), "rbc-hello.txt");
        Files.writeString(path, "hello");

        try (FileInputStream f = new FileInputStream(path.toFile())) {
            String text = new String(f.readAllBytes());
            System.out.println(text);
        }
        Files.delete(path);
    }
}
