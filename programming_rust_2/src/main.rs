use std::io;

fn main() {
    println!("Hello, world!");
}

// fn process_files_in_parallel(filenames: Vec<String>) -> io::Result<()> {
//     const NTHREADS: usize = 8;
//     let worklists = split_vec_into_cunks(filenames, NTHREADS);
//
//     // fork: spawn a thread to handle each chunk
//     let mut thread_handles = vec![];
//     for worklist in worklists {
//         thread_handles.push(
//             std::thread::spawn(|| {
//                 println!("hello");
//             })
//         );
//     }
//
//     for handle in thread_handles {
//         handle.join().unwrap()?;
//     }
//
//     Ok(())
// }

fn triangle(n: i32) -> i32 {
    (1..n + 1).fold(0, |sum, item| sum + item)
}