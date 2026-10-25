interface Vehicle {
    fun start(): String = "Vehicle::start"
}

interface Appliance {
    fun start(): String = "Appliance::start"
}

// Refused at the class, before anyone calls start().
class Car : Vehicle, Appliance

fun main() {
    println("unreachable: the file does not compile")
}
