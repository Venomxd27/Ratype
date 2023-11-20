use std::time::SystemTime;

pub fn get_accuracy(typing: &String, input: &String) -> f64 {
    let mut correct = 0;
    let original_chars = typing.chars().collect::<Vec<_>>();
    for (i, ch) in input.chars().enumerate() {
        if original_chars[i] == ch {
            correct += 1;
        }
    }

    (correct as f64 / input.len() as f64) * 100f64
}
pub fn get_wpm(start: SystemTime, input_writing: &str) -> f64 {
    let now = SystemTime::now();
    let elapsed = now.duration_since(start).unwrap().as_secs();
    let words = input_writing.split_whitespace().count();
    (words as f64 / elapsed as f64) * 60.0
}


//pub fn get_wpm(start_time: SystemTime, input: &String) -> f64 {
  //  let minutes = start_time.elapsed().unwrap().as_secs() as f64 / 60f64;
    //let words = input.split(" ").collect::<Vec<_>>().len() as f64;

//    words / minutes
//}
