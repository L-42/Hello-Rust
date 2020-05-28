// temperature-metric-conversions
// 相互转换摄氏与华氏温度。
// 生成 n 阶斐波那契数列。
// 打印圣诞颂歌 “The Twelve Days of Christmas” 的歌词，并利用歌曲中的重复部分（编写循环）。

fn main() {
    // let t = temperature_metric_conversions(20, "C2F");
    let t = temperature_metric_conversions(20.0, "F2C");
    let t = fibonacci(10);
    println!("{}", t);
    sing_a_christmas_song();
}

fn temperature_metric_conversions(num: f64, mode: &str) -> f64 {
    if mode == "C2F" {
        num * 1.8 + 32.0
    } else if mode == "F2C" {
        (num - 32.0) / 1.8
    } else {
        println!("THE INPUT MODE IS WRONG");
        0.0
    }
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

const NUM_MAP: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];
const EX_MAP: [&str; 12] = [
    "And a partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings, badam-pam-pam",
    "Six geese a laying",
    "Seven swans a swimming",
    "Eight maids a milking",
    "Nine ladies dancing",
    "Ten lords a leaping",
    "Eleven pipers piping",
    "12 drummers drumming",
];

fn sing_a_christmas_song() {
    for num in 0..12 {
        println!("On the {} day of Christmas", NUM_MAP[num]);
        println!("My true love gave to me");
        if num == 0 {
            println!("A partridge in a pear tree");
        } else {
            // FIXME: 为啥判断0，会触发unsafe？
            let mut temp = num + 1;
            while temp > 0 {
                println!("{}", EX_MAP[temp - 1]);
                temp = temp - 1;
            }
        }
        println!("")
    }
}
