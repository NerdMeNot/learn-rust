public class Main {
    enum Level {
        LOW(1), MID(5), HIGH(10);

        final int code;

        Level(int code) {
            this.code = code;
        }
    }

    static String label(Level l) {
        return switch (l) { // no default: the compiler checks every constant
            case LOW -> "low";
            case MID -> "mid";
            case HIGH -> "high";
        };
    }

    public static void main(String[] args) {
        Level m = Level.MID;
        System.out.println(m + " code " + m.code + " ordinal " + m.ordinal() + " label " + label(m));
        try {
            Level.valueOf("NOPE");
        } catch (IllegalArgumentException e) {
            System.out.println("valueOf(\"NOPE\"): " + e.getMessage());
        }
    }
}
