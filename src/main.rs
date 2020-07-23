use cached::proc_macro::cached;
use std::collections::HashMap;
use std::env;
use std::time::{Duration, Instant};

#[test]
fn test_each_version() {
    assert_eq!(backtrace_fib(20), 6765);
    assert_eq!(backtrace_memo_fib(&mut HashMap::new(), 20), 6765);
    assert_eq!(dynamic_fib(20), 6765);
    assert_eq!(better_dynamic_fib(20), 6765);
    assert_eq!(cached_fib(20), 6765);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} n (positive integer from 2 to 186)", args[0]);
        return;
    }

    let fib_num = args[1].parse::<u128>().unwrap();
    if fib_num < 2 {
        println!("Come on, Fibonacci Number {} is really boring", fib_num);
        return;
    } else if fib_num > 186 {
        println!("That's gonna get too big to calculate Phi, lets try something smaller");
        return;
    }

    println!("\nThe first time solving will be the slowest\n");
    solve_each(fib_num);
    println!("What about solving it a second or third time, anyone faster this time?\n");
    solve_each(fib_num);
    solve_each(fib_num);

    let answer = cached_fib(fib_num);
    let phi = (cached_fib(fib_num) as f64) / (cached_fib(fib_num - 1) as f64);
    println!(
        "By the way, Fibonacci Number {} is {} which (divided by Fib Num {}) approximates phi as {}",
        fib_num,
        answer,
        fib_num - 1,
        phi
    );
}

fn solve_each(fib_num: u128) {
    let now = Instant::now();
    let _ = backtrace_fib(fib_num);
    let elapsed = now.elapsed();
    print_results(fib_num, "simple backtracing/recursion", elapsed);

    let now = Instant::now();
    let _ = backtrace_memo_fib(&mut HashMap::new(), fib_num);
    let elapsed = now.elapsed();
    print_results(fib_num, "backtracing/recursion with memoization", elapsed);

    let now = Instant::now();
    let _ = dynamic_fib(fib_num);
    let elapsed = now.elapsed();
    print_results(
        fib_num,
        "dynamic programming with memoization via HashMap",
        elapsed,
    );

    let now = Instant::now();
    let _ = better_dynamic_fib(fib_num);
    let elapsed = now.elapsed();
    print_results(
        fib_num,
        "dynamic programming with memoization via tuple",
        elapsed,
    );

    let now = Instant::now();
    let _ = cached_fib(fib_num);
    let elapsed = now.elapsed();
    print_results(fib_num, "cached function", elapsed);

    println!();
}

fn print_results(fib_num: u128, desc: &str, elapsed: Duration) {
    println!(
        "  Solving fib:{} with {:49} took {:>15} ns",
        fib_num,
        desc,
        elapsed.as_nanos()
    );
}

// Simple recursion to backtrace our way backwards down the chain
// to 2 (which gets fixed answers for 0 and 1) and then unwinds to
// get the answer
fn backtrace_fib(fib_num: u128) -> u128 {
    if fib_num == 0 || fib_num == 1 {
        return fib_num;
    }
    backtrace_fib(fib_num - 1) + backtrace_fib(fib_num - 2)
}

// Simliar to above, but brings in a HashMap for memoization (weird
// that this is the only way to keep the memo around for future calls)
// otherwise, works the same as the backtrace before, it is justs faster
// because of the memoization
fn backtrace_memo_fib(memo: &mut HashMap<u128, u128>, fib_num: u128) -> u128 {
    match memo.get(&fib_num).map(|answer| answer.clone()) {
        Some(result) => result,
        None => {
            let result = match fib_num {
                0 | 1 => fib_num,
                n => backtrace_memo_fib(memo, n - 1) + backtrace_memo_fib(memo, n - 2),
            };
            memo.insert(fib_num, result.clone());
            result
        }
    }
}

// Loop thru the chain purposely instead of relying on recursion
// This code is less idiomatic of Rust though, uses an overly complex
// HashMap and keeps the entire chain in memory (see next method)
fn dynamic_fib(fib_num: u128) -> u128 {
    let mut memo = HashMap::new();
    memo.insert(0, 0);
    memo.insert(1, 1);
    match fib_num {
        0 | 1 => {} // already set
        n => {
            for i in 2..=n {
                let result = *memo.get(&(i - 1)).unwrap() + *memo.get(&(i - 2)).unwrap();
                memo.insert(i, result);
            }
        }
    };
    *memo.get(&fib_num).unwrap()
}

// Similar idea as above, but uses a tuple instead of a HashMap (duh)
// and constructs the chain of Fib numbers keeping ONLY the last 2
// as it works it's way to the fib_num
fn better_dynamic_fib(fib_num: u128) -> u128 {
    let mut memo = (0, 1);

    match fib_num {
        0 | 1 => fib_num,
        _ => {
            for _ in 2..=fib_num {
                memo = (memo.1, memo.0 + memo.1)
            }
            memo.1
        }
    }
}

// cached crate memoizes input->output of this function for us so
// we don't have to do any of it. We just do a simple recursive
// backtrace and everything is sped up because of memoization in
// the background
#[cached(size = 200)]
fn cached_fib(fib_num: u128) -> u128 {
    if fib_num == 0 || fib_num == 1 {
        return fib_num;
    }
    cached_fib(fib_num - 1) + cached_fib(fib_num - 2)
}
