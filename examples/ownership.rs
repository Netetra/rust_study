/*
struct Hoge<'b> {
    a: u32,
    b: char,
    c: &'b str,
    d: Vec<u8>
}
*/
#[derive(Debug)]
struct Hoge<'a> {
    huga: &'a i32
}

fn main() {
    let mut hoge = Hoge { huga: &20 };
    {
        let int = &40;
        hoge.huga = int;
    }
    println!("{:?}",hoge.huga); //エラー
}