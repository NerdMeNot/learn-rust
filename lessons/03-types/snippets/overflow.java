import java.math.BigDecimal;

public class Main {
    public static void main(String[] args) {
        int max = Integer.MAX_VALUE;
        System.out.println("MAX_VALUE + 1: " + (max + 1));
        try {
            Math.addExact(max, 1);
        } catch (ArithmeticException e) {
            System.out.println("addExact: " + e.getMessage());
        }

        byte b = (byte) 200;
        System.out.println("(byte) 200: " + b);
        System.out.println("as unsigned: " + Byte.toUnsignedInt(b));
        byte b1 = (byte) 100, b2 = (byte) 100;
        var sum = b1 + b2;
        System.out.println("byte + byte: " + sum + " has type " + ((Object) sum).getClass().getSimpleName());

        int big = 16_777_217;
        float f = big;
        System.out.println("int 16777217 to float: " + (int) f);
        long wide = 9_007_199_254_740_993L;
        double d = wide;
        System.out.println("long 2^53 + 1 to double: " + (long) d);

        System.out.println("(int) 1e10: " + (int) 1e10);
        System.out.println("(int) NaN: " + (int) Double.NaN);

        System.out.println("-7 / 2: " + (-7 / 2));
        System.out.println("-7 % 2: " + (-7 % 2));
        System.out.println("floorMod(-7, 2): " + Math.floorMod(-7, 2));
        System.out.println("floorMod(7, -2): " + Math.floorMod(7, -2));

        System.out.println("'😀'.length(): " + "😀".length());
        System.out.println("0.1 + 0.2 as BigDecimal: "
                + new BigDecimal("0.1").add(new BigDecimal("0.2")));
    }
}
