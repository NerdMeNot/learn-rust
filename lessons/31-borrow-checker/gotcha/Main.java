import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

class Item {
    final String name;
    final int count;

    Item(String name, int count) {
        this.name = name;
        this.count = count;
    }
}

class Inventory {
    final List<Item> items;
    final List<String> log = new ArrayList<>();

    Inventory(List<Item> items) {
        this.items = items;
    }

    void note(String text) {
        log.add(text);
    }

    void check() {
        for (Item item : items) {
            if (item.count == 0) {
                note("out of stock: " + item.name);
            }
        }
    }
}

public class Main {
    public static void main(String[] args) {
        Inventory inv = new Inventory(List.of(new Item("bolt", 0), new Item("nut", 12), new Item("washer", 0)));
        inv.check();
        System.out.println("log: " + inv.log.stream().map(s -> "\"" + s + "\"").collect(Collectors.joining(", ", "[", "]")));
    }
}
