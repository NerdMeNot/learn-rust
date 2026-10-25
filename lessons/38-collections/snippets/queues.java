import java.util.*;

public class Main {
    public static void main(String[] args) {
        PriorityQueue<Integer> pq = new PriorityQueue<>(List.of(3, 9, 4, 1));
        System.out.println("PriorityQueue peek: " + pq.peek());
        PriorityQueue<Integer> big = new PriorityQueue<>(Comparator.reverseOrder());
        big.addAll(List.of(3, 9, 4, 1));
        System.out.println("with reverseOrder, peek: " + big.peek());

        TreeMap<Integer, String> log = new TreeMap<>(Map.of(10, "boot", 25, "login", 40, "upload"));
        System.out.println("floorKey(30): " + log.floorKey(30) + ", ceilingKey(30): " + log.ceilingKey(30));
        SortedMap<Integer, String> view = log.subMap(10, 40);
        System.out.println("subMap(10, 40): " + view);
        log.put(30, "new");
        System.out.println("the same view after log.put(30): " + view);

        TreeMap<Integer, String> backwards = new TreeMap<>(Comparator.reverseOrder());
        backwards.putAll(log);
        System.out.println("TreeMap with a comparator, firstKey: " + backwards.firstKey());

        Set<String> names = new TreeSet<>(String.CASE_INSENSITIVE_ORDER);
        System.out.println("add Ana: " + names.add("Ana") + ", add ana: " + names.add("ana") + ", set: " + names);
        System.out.println("LinkedHashSet remembers first-seen order: " + new LinkedHashSet<>(List.of("b", "a", "b", "c")));

        Set<String> a = new TreeSet<>(List.of("ana", "bo", "cy"));
        Set<String> b = Set.of("bo", "cy", "dee");
        a.retainAll(b); // changes a in place; Rust's &a & &b builds a new set
        System.out.println("after a.retainAll(b), a is: " + a);

        ArrayDeque<String> q = new ArrayDeque<>();
        q.addLast("b");
        q.addFirst("a");
        System.out.println("ArrayDeque: " + q);
        System.out.println("pollFirst: " + q.pollFirst());
    }
}
