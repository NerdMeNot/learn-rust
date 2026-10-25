public class Main {
    static int clamp(int x) {
        if (x < 0) {
            return 0;
        }
        return x > 100 ? 100 : x;
    }

    public static void main(String[] args) {
        int n = 7;
        String label = n % 2 == 0 ? "even" : "odd";
        int w = 3;
        int h = 4;
        int area = w * h;
        System.out.println(n + " is " + label);
        System.out.println("area: " + area);
        System.out.println("clamp: " + clamp(-5) + " " + clamp(50) + " " + clamp(250));
    }
}
