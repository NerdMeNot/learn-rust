import java.nio.file.Files;
import java.nio.file.Path;

public class Main {
    static class ConfigError extends Exception {
        ConfigError(String message) { super(message); }
    }

    // The signature promises ConfigError, but readString throws IOException.
    static String readConfig(String path) throws ConfigError {
        return Files.readString(Path.of(path));
    }

    public static void main(String[] args) {
        try {
            System.out.println(readConfig("missing.conf"));
        } catch (ConfigError e) {
            System.out.println(e.getMessage());
        }
    }
}
