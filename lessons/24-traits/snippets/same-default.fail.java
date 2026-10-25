public class Main {
    interface Vehicle {
        default String start() {
            return "Vehicle::start";
        }
    }

    interface Appliance {
        default String start() {
            return "Appliance::start";
        }
    }

    // Refused at the class, before anyone calls start().
    static class Car implements Vehicle, Appliance {}

    public static void main(String[] args) {
        System.out.println("unreachable: the file does not compile");
    }
}
