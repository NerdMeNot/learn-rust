import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.concurrent.CopyOnWriteArrayList;

public class Main {
    public static void main(String[] args) {
        // 1. Remove 3 (the second-to-last) while looping: no exception, and 4 is never visited.
        List<Integer> list = new ArrayList<>(List.of(1, 2, 3, 4));
        List<Integer> seen = new ArrayList<>();
        for (int x : list) {
            seen.add(x);
            if (x == 3) list.remove(Integer.valueOf(3));
        }
        System.out.println("remove in a for-each: saw " + seen + ", list is " + list);

        // 2. Iterator.remove() is the supported way to remove while looping.
        list = new ArrayList<>(List.of(1, 2, 3, 4));
        seen.clear();
        for (Iterator<Integer> it = list.iterator(); it.hasNext(); ) {
            int x = it.next();
            seen.add(x);
            if (x == 3) it.remove();
        }
        System.out.println("Iterator.remove():    saw " + seen + ", list is " + list);

        // 3. CopyOnWriteArrayList: the loop walks a snapshot, so there is no exception.
        List<Integer> cow = new CopyOnWriteArrayList<>(List.of(1, 2, 3, 4));
        seen.clear();
        for (int x : cow) {
            seen.add(x);
            if (x == 2) cow.add(99);
        }
        System.out.println("CopyOnWriteArrayList: saw " + seen + ", list is " + cow);
    }
}
