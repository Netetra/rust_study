fn main() {
    let int = 10;
    let char = '!';
    let str = "Hello";
    let string = String::from("World");
    let float = 3.14;
    // 標準出力
    // print!(); 改行なし
    // println!(); 改行あり
    println!("{2} {3}{1} {0} {4}", int, char, str, string, float);

    // 標準エラー出力
    // eprint!(); 改行なし
    // eprintln!() 改行あり
    eprintln!("{2} {3}{1} {0} {4}", int, char, str, string, float);

    // ただフォーマットした文字列を返す
    let formated = format!("{2} {3}{1} {0} {4}", int, char, str, string, float);
    println!("{}", formated);
}
