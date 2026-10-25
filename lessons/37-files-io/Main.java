import java.io.BufferedReader;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.NoSuchFileException;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;

public class Main {
    public static void main(String[] args) throws IOException {
        Path path = Path.of(System.getProperty("java.io.tmpdir"), "rbc-notes.txt");

        // write a whole file at once (creates it, or empties it first)
        Files.writeString(path, "milk\neggs\n");

        // open for appending and add a line
        Files.writeString(path, "bread\n", StandardOpenOption.APPEND);

        // read the whole file into one String
        String all = Files.readString(path);
        System.out.println(all.length() + " chars");

        // read line by line; try-with-resources closes the reader
        try (BufferedReader reader = Files.newBufferedReader(path)) {
            int i = 1;
            for (String line; (line = reader.readLine()) != null; i++) {
                System.out.println(i + ": " + line);
            }
        }

        System.out.println("file name: \"" + path.getFileName() + "\"");

        // a missing file is a checked exception you have to catch or declare
        try {
            Files.readString(Path.of("/no/such/file.txt"));
        } catch (NoSuchFileException e) {
            System.out.println("error: " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }

        Files.delete(path);
    }
}
