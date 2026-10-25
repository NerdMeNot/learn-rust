import java.lang.reflect.Method;

interface Shape {
    int area();
}

class Square implements Shape {
    public int area() {
        return 9;
    }
}

class Rect implements Shape {
    public int area() {
        return 24;
    }
}

public class Main {
    static <T extends Shape> int twice(T s) {
        return 2 * s.area();
    }

    public static void main(String[] args) {
        System.out.println("twice(new Square()): " + twice(new Square()));
        System.out.println("twice(new Rect()): " + twice(new Rect()));
        int copies = 0;
        for (Method m : Main.class.getDeclaredMethods()) {
            if (m.getName().equals("twice")) {
                copies++;
                System.out.println("compiled parameter type: " + m.getParameterTypes()[0].getSimpleName());
            }
        }
        System.out.println("compiled copies of twice: " + copies);
    }
}
