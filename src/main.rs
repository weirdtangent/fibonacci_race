use cached::proc_macro::cached;
use std::collections::HashMap;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} n (positive integer to solve slow way)", args[0]);
        return;
    }

    let fib_num = args[1].parse::<u128>().unwrap();

    solve_each(fib_num);

    println!("\nWhat about solving it a second time, each way?\n");

    solve_each(fib_num);
}

fn solve_each(fib_num: u128) {
    // see how long it takes to solve what you provided
    let now = Instant::now();
    let _backtrace = backtrace_fib(fib_num);
    let backtrace_time = now.elapsed().as_nanos();
    println!(
        "  Solving fib:{} with simple backtracing and recursion            took {} ns",
        fib_num, backtrace_time
    );

    // find a fib number that takes longer to solve the backtrace_memo way
    let now = Instant::now();
    let _backtrace_memo = backtrace_memo_fib(&mut HashMap::new(), fib_num);
    let backtrace_memo_time = now.elapsed().as_nanos();
    println!(
        "  Solving fib:{} with backtracing and recursion using memoization took {} ns",
        fib_num, backtrace_memo_time
    );

    // find a fib number that takes longer to solve the dynamic way
    let now = Instant::now();
    let _dynamic = dynamic_fib(fib_num);
    let dynamic_time = now.elapsed().as_nanos();
    println!(
        "  Solving fib:{} with dynamic programming using memoization       took {} ns",
        fib_num, dynamic_time
    );

    // find a fib number that takes longer to solve the cached way
    let now = Instant::now();
    let _cached = cached_fib(fib_num);
    let cached_time = now.elapsed().as_nanos();
    println!(
        "  Solving fib:{} with cached programming using memoization        took {} ns",
        fib_num, cached_time
    );
}

fn backtrace_fib(fib_num: u128) -> u128 {
    if fib_num < 2 {
        return fib_num;
    }
    backtrace_fib(fib_num - 1) + backtrace_fib(fib_num - 2)
}

fn backtrace_memo_fib(memo: &mut HashMap<u128, u128>, fib_num: u128) -> u128 {
    match memo.get(&fib_num).map(|answer| answer.clone()) {
        Some(result) => result,
        None => {
            let result = match fib_num {
                0 => 0,
                1 => 1,
                n => backtrace_memo_fib(memo, n - 1) + backtrace_memo_fib(memo, n - 2),
            };
            memo.insert(fib_num, result.clone());
            result
        }
    }
}

fn dynamic_fib(fib_num: u128) -> u128 {
    let mut memo = HashMap::new();
    memo.insert(0, 0);
    memo.insert(1, 1);
    match fib_num {
        0 => {} // already set
        1 => {} // already set
        n => {
            for i in 2..=n {
                let result = *memo.get(&(i - 1)).unwrap() + *memo.get(&(i - 2)).unwrap();
                memo.insert(i, result);
            }
        }
    };
    *memo.get(&fib_num).unwrap()
}

#[cached(size = 100)]
fn cached_fib(fib_num: u128) -> u128 {
    if fib_num == 0 || fib_num == 1 {
        return fib_num;
    }
    cached_fib(fib_num - 1) + cached_fib(fib_num - 2)
}

#[test]
fn backtrace_works() {
    assert_eq!(backtrace_fib(20), 6765);
}

#[test]
fn backtrace_memo_works() {
    assert_eq!(backtrace_memo_fib(&mut HashMap::new(), 20), 6765);
}

#[test]
fn dynamic_works() {
    assert_eq!(dynamic_fib(20), 6765);
}

#[test]
fn cached_works() {
    assert_eq!(cached_fib(20), 6765);
}
