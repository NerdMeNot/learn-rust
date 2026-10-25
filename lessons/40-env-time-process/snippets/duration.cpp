#include <chrono>
#include <iostream>

int main() {
    using namespace std::chrono;
    // a duration is signed, so subtracting a later time from an earlier one is just negative
    auto early = steady_clock::now();
    auto later = early + seconds(1);
    std::cout << "early - later: " << duration_cast<milliseconds>(early - later).count() << " ms\n";

    auto wall_early = system_clock::now();
    auto wall_later = wall_early + seconds(1);
    std::cout << "system_clock early - later: "
              << duration_cast<milliseconds>(wall_early - wall_later).count() << " ms\n";
}
