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

// W is the before state, X is the input (if any)
pub trait CanPush<X> {
    type Result;
    fn push(self, val: X) -> Self::Result;
}
pub type Push<W,X> = <W as CanPush<X>>::Result;

pub trait CanPop {
    type Result;
    fn pop(self) -> Self::Result;
}
pub type Pop<W> = <W as CanPop>::Result;

pub trait CanShift {
    type Result;
    fn shift(self) -> Self::Result;
}
pub type Shift<W> = <W as CanShift>::Result;

pub trait CanUnshift<X> {
    type Result;
    fn unshift(self, val: X) -> Self::Result;
}
pub type Unshift<W,X> = <W as CanUnshift<X>>::Result;

macro_rules! mk_pushpop {
    ($firstlet:ident ; [$($letter:ident),*] ; $lastlet:ident ; $firstnum:tt ; [$($number:tt),*] ; $lastnum:tt) => {

        impl<X,$firstlet, $($letter,)* $lastlet> CanPush<X>
            for ($firstlet, $($letter,)* $lastlet)
        {
            type Result = ($firstlet, $($letter,)* $lastlet, X);
            fn push(self, val: X) -> Self::Result {
                ( self.$firstnum, $(self.$number,)* self.$lastnum, val )
            }
        }

        impl<$firstlet, $($letter,)* $lastlet> CanPop
            for ($firstlet, $($letter,)* $lastlet)
        {
            type Result = ($firstlet, $($letter,)*);
            fn pop(self) -> Self::Result {
                ( self.$firstnum, $(self.$number,)* )
            }
        }
        impl<$firstlet, $($letter,)* $lastlet> CanShift
            for ($firstlet, $($letter,)* $lastlet)
        {
            type Result = ($($letter,)* $lastlet);
            fn shift(self) -> Self::Result {
                ( $(self.$number,)* self.$lastnum )
            }
        }

        impl<X,$firstlet, $($letter,)* $lastlet> CanUnshift<X>
            for ($firstlet, $($letter,)* $lastlet)
        {
            type Result = (X, $firstlet, $($letter,)* $lastlet);
            fn unshift(self, val: X) -> Self::Result {
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

    fn do_push<W,X>(w:W, x:X) -> Push<W,X> where W:CanPush<X> { w.push(x) }
    fn do_pop<W>(w:W) -> Pop<W> where W:CanPop { w.pop() }
    fn do_shift<W>(w:W) -> Shift<W> where W:CanShift { w.shift() }
    fn do_unshift<W,X>(w:W, x:X) -> Unshift<W,X> where W:CanUnshift<X> { w.unshift(x) }

    #[test]
    fn main_test() {
        println!("{:?}", do_push((3,&"hello"), 4.5));
        println!("{:?}", do_pop((3,&"hello")));
        println!("{:?}", do_shift((3,&"hello")));
        println!("{:?}", do_unshift((3,&"hello"), 3));

        //let y = (1,&"hello",2);
        //println!(TypeId::of::<Us>(), TypeId::of::<(i32,&'static str,i32,)>());

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