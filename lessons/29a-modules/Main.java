public class Main {
    static class Shop {
        private static final int PRICE = 10;

        // private: hidden from other classes (and from other packages)
        private static int discountPercent(int qty) {
            return qty >= 10 ? 20 : 0;
        }

        public static int total(int qty) {
            int full = qty * PRICE;
            return full - full * discountPercent(qty) / 100;
        }
    }

    public static void main(String[] args) {
        System.out.println("3 items: " + Shop.total(3));
        System.out.println("12 items: " + Shop.total(12));
    }
}
