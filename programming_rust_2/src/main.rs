use std::io;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

// just stub
fn process_files(filenames: Vec<String>, glossary_child: &HashMap<i32, i32>) {}

fn process_files_in_parallel(filenames: Vec<String>, glossary: std::sync::Arc<HashMap<i32, i32>>) -> io::Result<()> {
    const NTHREADS: usize = 8;
    // let worklists = split_vec_into_cunks(filenames, NTHREADS);
    let worklists: Vec<Vec<String>> = vec![];

    // fork: spawn a thread to handle each chunk
    let mut thread_handles = vec![];
    for worklist in worklists {
        // This call to .clone() only clones the Arc and bumps the
        // reference count. It does not clone the GigabyteMap.
        let glossary_clone = glossary.clone();
        thread_handles.push(
            std::thread::spawn(move || {
                process_files(worklist, &glossary_clone);
            })
        );
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    Ok(())
}

fn triangle(n: i32) -> i32 {
    (1..n + 1).fold(0, |sum, item| sum + item)
}