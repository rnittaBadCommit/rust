use i32 as I32;
use std::collections::HashMap;

pub fn f() {
    let mut scores = HashMap::new();
    scores.insert("abc", 10);

    scores.entry("abc").or_insert(50);
    let mut x = scores.entry("aaa");
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        pub fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

        mod a {
            pub fn f() {}
            pub fn g() {}
        }

        mod b{
            pub fn f(){
                super::a::f();
                crate::front_of_house::serving::a::f();
            }
        }
    }
    
    pub fn f() {
        serving::take_order();
    }
}


pub fn eat_at_restaurant() {
    let x: I32 = 12;
    println!("{x}");
    let s = "abc";
    let s2 = s.to_string();
    let s3 = "abc".to_string();
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
    front_of_house::f(); 
}