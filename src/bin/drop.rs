struct Tag {
    num: usize,
}

impl Drop for Tag {
    fn drop(&mut self) {
        println!("Tag{}: drop", self.num);
    }
}

fn main() {
    println!("--------start--------");
    Tag { num: 1 };
    let _ = Tag { num: 2 };
    let x = Tag { num: 3 };
    let y = Tag { num: 4 };
    {
        let _z = Tag { num: 5 };
        let _ = Tag { num: 6 };
        println!("-----start scope-----");
        println!("Tag{}: print", y.num);
        Tag { num: 7 };
        println!("-----end scope-----");
    }
    println!("Tag{}: print", x.num);
    println!("--------end--------");
}
