public class Main {
    interface Speaker {
        String speak();

        default String shout() {   // default method
            return speak().toUpperCase();
        }
    }

    record Dog() implements Speaker {
        public String speak() {
            return "woof";
        }
    }

    record Robot(int id) implements Speaker {
        public String speak() {
            return "beep " + id;
        }
    }

    static void introduce(Speaker s) {
        System.out.println(s.speak() + " / " + s.shout());
    }

    public static void main(String[] args) {
        introduce(new Dog());
        introduce(new Robot(7));
    }
}
