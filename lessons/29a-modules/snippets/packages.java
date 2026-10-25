import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

public class Main {
    static void write(Path root, String path, String text) throws IOException {
        Path full = root.resolve(path);
        Files.createDirectories(full.getParent());
        Files.writeString(full, text);
    }

    static void compile(Path root, String label, String... args) throws Exception {
        System.out.println(label);
        String[] cmd = new String[args.length + 3];
        cmd[0] = "javac";
        cmd[1] = "-d";
        cmd[2] = "out";
        System.arraycopy(args, 0, cmd, 3, args.length);
        Process p = new ProcessBuilder(cmd).directory(root.toFile()).redirectErrorStream(true).start();
        String output = new String(p.getInputStream().readAllBytes());
        int code = p.waitFor();
        System.out.print(output);
        if (code == 0) {
            System.out.println("  ok");
        }
    }

    public static void main(String[] args) throws Exception {
        Path root = Files.createTempDirectory("javapkg");
        // No modifier on `rate`: visible to the package, not to the world.
        write(root, "shop/Shop.java", """
                package shop;

                public class Shop {
                    static int rate() { return 20; }
                    public static int total(int q) { return q * 10 - q * 10 * rate() / 100; }
                }
                """);
        write(root, "shop/Audit.java", """
                package shop;

                public class Audit {
                    public static int check() { return Shop.rate(); }
                }
                """);
        write(root, "app/App.java", """
                package app;

                import shop.Shop;

                public class App {
                    public static int run() { return Shop.rate(); }
                }
                """);
        compile(root, "1. a class in the same package calls rate()", "shop/Shop.java", "shop/Audit.java");
        compile(root, "2. a class in another package calls rate()", "shop/Shop.java", "app/App.java");

        // `protected`: the package, plus subclasses elsewhere.
        write(root, "shop/Base.java", """
                package shop;

                public class Base {
                    protected static int rate() { return 20; }
                }
                """);
        write(root, "app/Child.java", """
                package app;

                public class Child extends shop.Base {
                    public static int run() { return rate(); }
                }
                """);
        write(root, "app/Stranger.java", """
                package app;

                public class Stranger {
                    public static int run() { return shop.Base.rate(); }
                }
                """);
        compile(root, "3. a subclass in another package calls a protected method", "shop/Base.java", "app/Child.java");
        compile(root, "4. a class that is not a subclass calls it", "shop/Base.java", "app/Stranger.java");

        // A public class in a package its module does not export.
        write(root, "mods/shop.lib/module-info.java", """
                module shop.lib {
                    exports shop.api;
                }
                """);
        write(root, "mods/shop.lib/shop/internal/Secret.java", """
                package shop.internal;

                public class Secret {
                    public static int rate() { return 20; }
                }
                """);
        write(root, "mods/shop.lib/shop/api/Api.java", """
                package shop.api;

                public class Api {
                    public static int total(int q) { return q * 10 - q * 10 * shop.internal.Secret.rate() / 100; }
                }
                """);
        write(root, "mods/shop.app/module-info.java", """
                module shop.app {
                    requires shop.lib;
                }
                """);
        write(root, "mods/shop.app/app/App.java", """
                package app;

                public class App {
                    public static int run() { return shop.internal.Secret.rate(); }
                }
                """);
        compile(root, "5. another module uses a public class in a package that is not exported",
                "--module-source-path", "mods",
                "mods/shop.lib/module-info.java", "mods/shop.lib/shop/internal/Secret.java",
                "mods/shop.lib/shop/api/Api.java", "mods/shop.app/module-info.java",
                "mods/shop.app/app/App.java");
    }
}
