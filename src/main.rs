fn main() {
    let sentence :String = "I love NewYork".to_string();
    // let sentence :&str = "I love NewYork";

    for i in 0..sentence.len() as usize {
      println!("{}", sentence.chars().nth(i).unwrap());
    }
}
