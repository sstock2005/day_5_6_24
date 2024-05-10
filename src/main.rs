
// NOT doing tar, working on something else
// splits array to find max cocurrently with short-lived thread

fn calculate_max(array: &[i32]) -> Option<i32> 
{
    const THRESHOLD: usize = 2;

    if array.len() <= THRESHOLD
    {
        return array.iter().cloned().max();
    }

    let mid = array.len() / 2;
    let (left, right) = array.split_at(mid);

    crossbeam::scope(|s| {
        let thread_left = s.spawn(|_| calculate_max(left));
        let thread_right = s.spawn(|_| calculate_max(right));

        let max_left = thread_left.join().unwrap();
        let max_right = thread_right.join().unwrap();

        Some(max_left.max(max_right)?)
    }).unwrap()
}

fn main()
{
    let array = &[1, 25, -4, 10];
    let max = calculate_max(array);

    assert_eq!(max, Some(25));

    println!("success!");
}