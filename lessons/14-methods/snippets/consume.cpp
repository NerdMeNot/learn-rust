#include <iostream>
#include <string>
#include <utility>

struct Account {
    std::string owner;
    unsigned balance;

    // The closest thing to Rust's `self`: callable only on an rvalue.
    std::string close() && { return std::move(owner) + " closed"; }
};

int main() {
    Account acct{"ana", 75};
    std::string note = std::move(acct).close();
    std::cout << "note: " << note << "\n";
    std::cout << "owner afterwards: '" << acct.owner << "'\n";
    std::cout << "balance afterwards: " << acct.balance << "\n";
}
