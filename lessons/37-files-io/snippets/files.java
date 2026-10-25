import java.io.BufferedOutputStream;
import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.FileOutputStream;
import java.io.FileWriter;
import java.io.IOException;
import java.io.OutputStream;
import java.nio.file.Files;
import java.nio.file.Path;

public class Main {
    // how many write system calls this process has made so far (Linux: the syscw line)
    static long writeCalls() throws IOException {
        for (String l : Files.readAllLines(Path.of("/proc/self/io"))) {
            if (l.startsWith("syscw:")) return Long.parseLong(l.substring(6).trim());
        }
        return -1;
    }

    static long thousandLines(OutputStream out) throws IOException {
        long before = writeCalls();
        for (int i = 0; i < 1000; i++) out.write(("line " + i + "\n").getBytes());
        long calls = writeCalls() - before;
        out.close();
        return calls;
    }

    public static void main(String[] args) throws IOException {
        try {
            Files.readString(Path.of("/no/such/file.txt"));
        } catch (IOException e) {
            System.out.println(e.getClass().getSimpleName() + ": " + e.getMessage());
        }

        Path path = Path.of(System.getProperty("java.io.tmpdir"), "lr-fio-java.txt");
        Files.write(path, "good\nbad ÿ here\n".getBytes(java.nio.charset.StandardCharsets.ISO_8859_1));
        try {
            Files.readString(path);
        } catch (IOException e) {
            System.out.println(e.getClass().getSimpleName() + ": " + e.getMessage());
        }

        // the reader is closed when the block ends, even on an exception
        try (BufferedReader r = Files.newBufferedReader(path, java.nio.charset.StandardCharsets.ISO_8859_1)) {
            System.out.println("first line: " + r.readLine());
        }

        // FileOutputStream is unbuffered like a Rust File; BufferedOutputStream is a BufWriter
        long direct = thousandLines(new FileOutputStream(path.toFile()));
        long buffered = thousandLines(new BufferedOutputStream(new FileOutputStream(path.toFile())));
        System.out.println("FileOutputStream: " + direct + " write calls, BufferedOutputStream: " + buffered);
        Files.delete(path);

        // close() flushes the buffer, and try-with-resources reports a failure there.
        // /dev/full refuses every byte, like a full disk (Linux).
        try (BufferedWriter w = new BufferedWriter(new FileWriter("/dev/full"))) {
            w.write("data");
        } catch (IOException e) {
            System.out.println("close() reported: " + e.getMessage());
        }

        System.out.println("resolve(absolute): " + Path.of("/data").resolve("/etc/hosts"));
    }
}
