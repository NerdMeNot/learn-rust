#include <iostream>
#include <thread>

int main() {
    std::cout << "main: starting the worker\n";
    {
        std::jthread t([] { std::cout << "worker ran\n"; });
    }   // ~jthread joins, so the worker has finished here
    std::cout << "main: after the block\n";
}
