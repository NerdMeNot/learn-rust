class Res implements AutoCloseable {
    final String name;

    Res(String name) {
        this.name = name;
        System.out.println("open " + name);
    }

    @Override
    public void close() {
        System.out.println("close " + name);
    }
}

public class Cleanup {
    public static void main(String[] args) {
        try (Res a = new Res("a"); Res b = new Res("b")) {
            System.out.println("working");
        }
        Res c = new Res("c");
        System.out.println("done");
    }
}
