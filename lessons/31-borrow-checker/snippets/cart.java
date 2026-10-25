import java.util.ArrayList;
import java.util.List;

class Cart {
    private final List<String> items = new ArrayList<>();

    List<String> items() {
        return items;
    }

    int size() {
        return items.size();
    }
}

public class Main {
    public static void main(String[] args) {
        Cart cart = new Cart();
        List<String> view = cart.items();
        view.add("tea");
        System.out.println("cart size: " + cart.size());
    }
}
