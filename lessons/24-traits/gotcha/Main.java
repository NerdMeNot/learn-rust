public class Main {
    interface Speaker {
        String speak();
    }

    record Robot(int id) {
        String speak() {
            return "beep " + id;
        }
    }

    static void introduce(Speaker s) {
        System.out.println(s.speak());
    }

    public static void main(String[] args) {
        introduce(new Robot(7));
    }
}
