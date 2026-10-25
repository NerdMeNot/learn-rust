public class Main {
    static class Shop {
        private static int discountPercent(int qty) {
            return qty >= 10 ? 20 : 0;
        }

        public static int total(int qty) {
            int full = qty * 10;
            return full - full * discountPercent(qty) / 100;
        }
    }

    public static void main(String[] args) {
        System.out.println("total: " + Shop.total(12));
        System.out.println("discount: " + Shop.discountPercent(12) + "%");
    }
}
