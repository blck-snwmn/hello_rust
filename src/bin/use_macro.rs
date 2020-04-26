
macro_rules! show{
    ( $( $x:tt ),* ) => {
        // 生成結果が複数行になるため
        // `{`, `}` が必要そう
        {$( println!("{}",$x); )*}
    }
}

macro_rules! show2{
    ( $( $x:expr ),* ) => {
        {$( println!("{}",$x); )*}
    }
}
macro_rules! double{
    ($x:expr)=>($x*2)
}

// 繰り返し `;` 区切り
macro_rules! doubles{
    ( $( $x:expr );* ) => {
        &[$($x),*]
    }
}

fn main() {
    show!(1, 2, 3, 4);  // let ar = [1, 2, 3, 4];
    show2!(1, 2, 3, 4);  // let ar = [1, 2, 3, 4];
    println!("double!(3)={}", double!(3));
    
    let ar = doubles!(3;10);
    for v in ar {
        println!("doubles! gen {}", v);
    }
}