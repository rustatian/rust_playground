#[allow(dead_code)]
#[allow(unused_variables)]
pub fn stringsss() {
    let question = "How are you??";
    let person: String = "Bob".to_string();
//    let namaste = String::from(2423);

//    println!("{}! {} {}", namaste, question, person);
}

#[allow(dead_code)]
pub fn sanitize(s: String) -> String {
    let s = s.trim();
    let s = s.replace(" ", "_");
    s
}