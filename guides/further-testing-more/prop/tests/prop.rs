use avg::{average, average_naive};
use proptest::prelude::*;
use proptest::test_runner::{Config, RngSeed, TestError, TestRunner};

// A fixed seed and no regression file make this page's run repeatable. A real project
// would leave both out (see the page).
fn config() -> ProptestConfig {
    ProptestConfig {
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(42),
        ..ProptestConfig::default()
    }
}

proptest! {
    #![proptest_config(config())]

    // 256 random vectors (the default), 1 to 7 samples each, any i32.
    #[test]
    fn average_is_between_min_and_max(samples in prop::collection::vec(any::<i32>(), 1..8)) {
        let avg = average(&samples).unwrap();
        prop_assert!(avg >= *samples.iter().min().unwrap());
        prop_assert!(avg <= *samples.iter().max().unwrap());
    }
}

// The same property for the naive version, run by hand so that the test can pass while
// it prints what proptest found.
#[test]
fn naive_average_is_caught_and_shrunk() {
    // proptest catches the overflow panics itself; the hook only hides their messages.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let mut runner = TestRunner::new(Config { ..config() });
    let result = runner.run(&prop::collection::vec(any::<i32>(), 1..8), |samples| {
        let avg = average_naive(&samples).unwrap();
        prop_assert!(avg >= *samples.iter().min().unwrap());
        Ok(())
    });
    std::panic::set_hook(default_hook);
    match result {
        Err(TestError::Fail(reason, minimal)) => {
            println!("proptest says: {reason}");
            println!("minimal failing input: {minimal:?}");
        }
        other => panic!("expected a failure, got {other:?}"),
    }
}
