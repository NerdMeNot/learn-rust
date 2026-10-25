public class Main {
    static int add(int a, int b) {
        return a + b;
    }

    static void greet(String name) {   // void means it returns nothing
        System.out.println("hi " + name);
    }

    public static void main(String[] args) {
        System.out.println(add(2, 3));
        greet("kotlin");
    }
}
