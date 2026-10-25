use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, TryLockError};
use std::thread;

pub struct Bank {
    accounts: Vec<Mutex<i64>>,
    requests: Mutex<Vec<usize>>,
    holding: AtomicUsize,
    most_held: AtomicUsize,
    careful: bool,
}

/// An account lock that counts how many are held at once. Use it like a `MutexGuard`.
pub struct Held<'a> {
    guard: MutexGuard<'a, i64>,
    bank: &'a Bank,
}

impl Deref for Held<'_> {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.guard
    }
}

impl DerefMut for Held<'_> {
    fn deref_mut(&mut self) -> &mut i64 {
        &mut self.guard
    }
}

impl Drop for Held<'_> {
    fn drop(&mut self) {
        self.bank.holding.fetch_sub(1, Ordering::SeqCst);
    }
}

impl Bank {
    pub fn new(balances: &[i64]) -> Bank {
        Bank {
            accounts: balances.iter().map(|&b| Mutex::new(b)).collect(),
            requests: Mutex::new(Vec::new()),
            holding: AtomicUsize::new(0),
            most_held: AtomicUsize::new(0),
            careful: false,
        }
    }

    /// Like `new`, but `lock` panics instead of waiting when an account is already locked.
    /// The single-threaded checks use it, so a self-deadlock is a message, not a hang.
    pub fn careful(balances: &[i64]) -> Bank {
        Bank {
            careful: true,
            ..Bank::new(balances)
        }
    }

    /// Lock account `i`. Always use this, so the checks can see your order and how many
    /// locks you hold at the same time.
    fn lock(&self, i: usize) -> Held<'_> {
        self.requests.lock().unwrap().push(i);
        let guard = if self.careful {
            match self.accounts[i].try_lock() {
                Ok(guard) => guard,
                Err(TryLockError::WouldBlock) => {
                    panic!("account {i} is already locked: waiting for it would never end")
                }
                Err(TryLockError::Poisoned(_)) => panic!("account {i} is poisoned"),
            }
        } else {
            self.accounts[i].lock().unwrap()
        };
        let now = self.holding.fetch_add(1, Ordering::SeqCst) + 1;
        self.most_held.fetch_max(now, Ordering::SeqCst);
        Held { guard, bank: self }
    }

    pub fn balance(&self, i: usize) -> i64 {
        *self.accounts[i].lock().unwrap()
    }

    /// The sum of all balances, read while holding every account lock (in index order).
    pub fn total(&self) -> i64 {
        let all: Vec<_> = self.accounts.iter().map(|a| a.lock().unwrap()).collect();
        all.iter().map(|g| **g).sum()
    }

    fn take_requests(&self) -> Vec<usize> {
        std::mem::take(&mut *self.requests.lock().unwrap())
    }

    /// The most account locks held at the same time since the last call.
    fn take_most_held(&self) -> usize {
        self.most_held.swap(0, Ordering::SeqCst)
    }

    /// Move `amount` from account `from` to account `to`.
    /// - Not enough money: `Err("insufficient funds")` and nothing changes.
    /// - `from == to` is allowed and changes nothing; that account must not be locked twice.
    /// - Two transfers in opposite directions must never wait for each other forever:
    ///   when two accounts are needed, lock the lower index first.
    /// - Nobody may see the money gone from one account and not yet in the other:
    ///   hold both locks until both balances are changed.
    pub fn transfer(&self, from: usize, to: usize, amount: i64) -> Result<(), String> {
        todo!("lock in index order, then check the balance, then move the money")
    }
}

fn main() {
    // The lock order is the same whichever way the money goes, and both locks are held
    // while the money moves.
    let bank = Bank::careful(&[100, 50, 0]);
    assert_eq!(bank.transfer(0, 1, 30), Ok(()));
    assert_eq!(bank.take_requests(), [0, 1]);
    assert_eq!(
        bank.take_most_held(),
        2,
        "hold both locks while moving the money"
    );
    assert_eq!(bank.transfer(1, 0, 10), Ok(()));
    assert_eq!(bank.take_requests(), [0, 1], "lock the lower index first");
    assert_eq!(
        bank.take_most_held(),
        2,
        "hold both locks while moving the money"
    );
    assert_eq!((bank.balance(0), bank.balance(1)), (80, 70));

    // Not enough money: an error, and nothing moved.
    assert_eq!(
        bank.transfer(2, 0, 1),
        Err("insufficient funds".to_string())
    );
    assert_eq!(
        (bank.balance(0), bank.balance(1), bank.balance(2)),
        (80, 70, 0)
    );
    bank.take_requests();

    // The same account twice: no change, and the account is never locked twice.
    assert_eq!(bank.transfer(1, 1, 5), Ok(()));
    assert!(
        bank.take_requests().len() <= 1,
        "lock a single account at most once"
    );
    assert_eq!(bank.balance(1), 70);

    // Opposite directions from many threads while a reader keeps adding up all the
    // accounts. The total is 3000 every time, so nobody may see money in flight.
    let bank = Arc::new(Bank::new(&[1000, 1000, 1000]));
    let done = Arc::new(AtomicBool::new(false));
    let reader = {
        let (bank, done) = (Arc::clone(&bank), Arc::clone(&done));
        thread::spawn(move || {
            let mut sums = 0;
            while !done.load(Ordering::SeqCst) {
                assert_eq!(
                    bank.total(),
                    3000,
                    "money was missing in the middle of a transfer"
                );
                sums += 1;
            }
            sums
        })
    };
    let handles: Vec<_> = (0..4)
        .map(|t| {
            let bank = Arc::clone(&bank);
            thread::spawn(move || {
                for i in 0..2000 {
                    let (from, to) = if (i + t) % 2 == 0 { (0, 2) } else { (2, 0) };
                    let _ = bank.transfer(from, to, 1);
                    let _ = bank.transfer(1, if i % 2 == 0 { 0 } else { 2 }, 1);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    done.store(true, Ordering::SeqCst);
    assert!(reader.join().unwrap() > 0);
    assert_eq!(bank.total(), 3000);
    println!("all checks passed");
}
