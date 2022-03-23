mod regex;

fn main() {
    let src = "a.c$";
    let regex = regex::Regex::compile(src);
    regex.dump();

    println!("{}", "-".repeat(50));

    println!("regex: {}", src);

    let inputs = ["bc", "abc", "acc", "abcd"];

    for input in inputs {
        println!("{} => {}", input, regex.match_str(input));
    }
}
