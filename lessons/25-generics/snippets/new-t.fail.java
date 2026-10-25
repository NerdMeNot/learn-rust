public class Main {
    static <T> T make() {
        return new T();
    }

    public static void main(String[] args) {
        make();
    }
}
