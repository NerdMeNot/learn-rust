public class Main {
    public static void main(String[] args) {
        assert 1 + 1 == 3 : "arithmetic broke";
        System.out.println("assert did not stop the program");
    }
}
