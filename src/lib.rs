use std::fmt;
pub trait Join { fn join(&self, sep: &str) -> String; }
macro_rules! mk_join {
    ([$($letter:ident),*] ; [$($number:tt),*] ; $lastnum:tt ; $fmt:expr) => {
        impl<$($letter,)*> Join for ($($letter,)*) where $($letter : fmt::Display,)* {
            fn join(&self, sep: &str) -> String { format!($fmt, $(self.$number, sep,)* self.$lastnum) }
        }
    }
}
mk_join!([A,B] ; [0] ; 1 ; "{}{}{}");
mk_join!([A,B,C] ; [0,1] ; 2 ; "{}{}{}{}{}");
mk_join!([A,B,C,D] ; [0,1,2] ; 3 ; "{}{}{}{}{}{}{}");
mk_join!([A,B,C,D,E] ; [0,1,2,3] ; 4 ; "{}{}{}{}{}{}{}{}{}");
mk_join!([A,B,C,D,E,F] ; [0,1,2,3,4] ; 5 ; "{}{}{}{}{}{}{}{}{}{}{}");
mk_join!([A,B,C,D,E,F,G] ; [0,1,2,3,4,5] ; 6 ; "{}{}{}{}{}{}{}{}{}{}{}{}{}");
mk_join!([A,B,C,D,E,F,G,H] ; [0,1,2,3,4,5,6] ; 7 ; "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}");
mk_join!([A,B,C,D,E,F,G,H,I] ; [0,1,2,3,4,5,6,7] ; 8 ; "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}");
mk_join!([A,B,C,D,E,F,G,H,I,J] ; [0,1,2,3,4,5,6,7,8] ; 9 ; "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}");
mk_join!([A,B,C,D,E,F,G,H,I,J,K] ; [0,1,2,3,4,5,6,7,8,9] ; 10 ; "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}");
mk_join!([A,B,C,D,E,F,G,H,I,J,K,L] ; [0,1,2,3,4,5,6,7,8,9,10] ; 11 ; "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}");


pub trait Push<X,Y> { fn push(self, val: X) -> Y; }
pub trait Pop<Y> { fn pop(self) -> Y; }
pub trait Shift<Y> { fn shift(self) -> Y; }
pub trait Unshift<X,Y> { fn unshift(self, val: X) -> Y; }

macro_rules! mk_pushpop {
    ($firstlet:ident ; [$($letter:ident),*] ; $lastlet:ident ; $firstnum:tt ; [$($number:tt),*] ; $lastnum:tt) => {
        impl<X,$firstlet, $($letter,)* $lastlet> Push<X,($firstlet, $($letter,)* $lastlet, X)>
            for ($firstlet, $($letter,)* $lastlet)
        {
            fn push(self, val: X) -> ($firstlet, $($letter,)* $lastlet,X) {
                ( self.$firstnum, $(self.$number,)* self.$lastnum, val )
            }
        }
        impl<$firstlet, $($letter,)* $lastlet> Pop<($firstlet, $($letter,)*)>
            for ($firstlet, $($letter,)* $lastlet)
        {
            fn pop(self) -> ($firstlet, $($letter),*) {
                ( self.$firstnum, $(self.$number,)* )
            }
        }
        impl<$firstlet, $($letter,)* $lastlet> Shift<($($letter,)* $lastlet)>
            for ($firstlet, $($letter,)* $lastlet)
        {
            fn shift(self) -> ($($letter,)* $lastlet) {
                ( $(self.$number,)* self.$lastnum )
            }
        }
        impl<X,$firstlet, $($letter,)* $lastlet> Unshift<X,(X, $firstlet, $($letter,)* $lastlet)>
            for ($firstlet, $($letter,)* $lastlet)
        {
            fn unshift(self, val: X) -> (X, $firstlet, $($letter,)* $lastlet) {
                ( val, self.$firstnum, $(self.$number,)* self.$lastnum )
            }
        }
    }
}
mk_pushpop!(A ; [] ; B ; 0 ; [] ; 1);
mk_pushpop!(A ; [B] ; C ; 0 ; [1] ; 2);
mk_pushpop!(A ; [B,C] ; D ; 0 ; [1,2] ; 3);
mk_pushpop!(A ; [B,C,D] ; E ; 0 ; [1,2,3] ; 4);
mk_pushpop!(A ; [B,C,D,E] ; F ; 0 ; [1,2,3,4] ; 5);
mk_pushpop!(A ; [B,C,D,E,F] ; G ; 0 ; [1,2,3,4,5] ; 6);
mk_pushpop!(A ; [B,C,D,E,F,G] ; H ; 0 ; [1,2,3,4,5,6] ; 7);
mk_pushpop!(A ; [B,C,D,E,F,G,H] ; I ; 0 ; [1,2,3,4,5,6,7] ; 8);
mk_pushpop!(A ; [B,C,D,E,F,G,H,I] ; J ; 0 ; [1,2,3,4,5,6,7,8] ; 9);
mk_pushpop!(A ; [B,C,D,E,F,G,H,I,J] ; K ; 0 ; [1,2,3,4,5,6,7,8,9] ; 10);
mk_pushpop!(A ; [B,C,D,E,F,G,H,I,J,K] ; L ; 0 ; [1,2,3,4,5,6,7,8,9,10] ; 11);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_test() {
        println!("{}", ("hello", "world").join(" ") );
        println!("{}", (1, 1.1).join(" ") );
        println!("{}", ("hello", 1.1).join(" ") );
        println!("{}", ("hello", 1.1, 1, "world").join(",") );

        let x = (1,"hi");
        let x = x.push(3.3);
        let x = x.push("test");
        println!("{}", x.join(" "));

        let x = x.pop();
        println!("{}", x.join(" "));

        let x = x.unshift("before");
        println!("{:?}", x);

        let x = x.shift().shift();
        println!("{:?}", x);
    }
}