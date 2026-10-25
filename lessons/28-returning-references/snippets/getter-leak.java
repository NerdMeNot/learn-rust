import java.util.*;

public class Main {
    static class Team {
        private final List<String> names = new ArrayList<>(List.of("ana", "bo"));

        List<String> getNames() {
            return names; // the caller gets the list the object keeps
        }

        int size() {
            return names.size();
        }
    }

    public static void main(String[] args) {
        var team = new Team();
        team.getNames().clear();
        System.out.println("size: " + team.size());
    }
}
