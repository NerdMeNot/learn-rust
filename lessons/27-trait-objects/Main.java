import java.util.List;

interface Shape {
    String name();
    int area();
}

class Square implements Shape {
    private final int side;
    Square(int side) { this.side = side; }
    public String name() { return "square"; }
    public int area() { return side * side; }
}

class Rect implements Shape {
    private final int w, h;
    Rect(int w, int h) { this.w = w; this.h = h; }
    public String name() { return "rect"; }
    public int area() { return w * h; }
}

public class Main {
    public static void main(String[] args) {
        List<Shape> shapes = List.of(new Square(3), new Rect(4, 6));

        int total = 0;
        for (Shape s : shapes) {
            System.out.println(s.name() + ": " + s.area());
            total += s.area();
        }
        System.out.println("total: " + total);
    }
}
