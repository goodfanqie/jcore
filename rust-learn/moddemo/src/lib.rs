mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {
            super::hosting::add_to_waitlist();
            // super类似于文件操作中的../
            // 它可以访问上一级父模块中的内容
            // 以及与其父模块同级的叔叔模块
        }

        pub fn serve_order() {
            super::super::eat_at_restaurant();
            // 连上两级
        }

        fn take_payment() {}
    }
}
use front_of_house::serving;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
    serving::serve_order();
}
