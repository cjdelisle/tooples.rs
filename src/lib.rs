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


pub trait CanApply<F>: CanApplyMut<F> {
    fn apply_to(self, f:&F) -> Self::Output;
}
pub trait CanApplyMut<F>: CanApplyOnce<F> {
    fn apply_to_mut(self, f:&mut F) -> Self::Output;
}
pub trait CanApplyOnce<F> {
    type Output;
    fn apply_to_once(self, f:F) -> Self::Output;
}

macro_rules! mk_funcs {
    (
        TWO;
        $firstlet:ident;
        [$($letter:ident),*];
        $lastlet:ident;
        $firstnum:tt;
        [$($number:tt),*];
        $lastnum:tt
    ) => {
        mk_funcs!(
            ONE;
            [$firstlet, $($letter,)* $lastlet];
            [$($letter,)* $lastlet];
            [$firstlet $(,$letter)*];
            [$firstnum, $($number,)* $lastnum];
            [$($number,)* $lastnum];
            [$firstnum $(,$number)*]
        );
    };
    (
        ONE;
        [$($letters:ident),*];
        [$($lnofirst:ident),*];
        [$($lnolast:ident),*];
        [$($numbers:tt),*];
        [$($nnofirst:tt),*];
        [$($nnolast:tt),*]
    ) => {
        impl<X,$($letters,)*> CanPush<X> for ($($letters,)*) {
            type Result = ($($letters,)* X,);
            fn push(self, val: X) -> Self::Result { ( $(self.$numbers,)* val, ) }
        }
        impl<$($letters,)*> CanPop for ($($letters,)*) {
            type Result = ($($lnolast,)*);
            fn pop(self) -> Self::Result { ( $(self.$nnolast,)* ) }
        }
        impl<$($letters,)*> CanShift for ($($letters,)*) {
            type Result = ($($lnofirst,)*);
            fn shift(self) -> Self::Result { ( $(self.$nnofirst,)* ) }
        }
        impl<X,$($letters,)*> CanUnshift<X> for ($($letters,)*) {
            type Result = (X, $($letters,)*);
            fn unshift(self, val: X) -> Self::Result {
                ( val, $(self.$numbers,)* )
            }
        }

        impl<Fun,Z,$($letters,)*> CanApply<Fun> for ($($letters,)*) where Fun:Fn($($letters,)*)->Z {
            fn apply_to(self, f:&Fun) -> Self::Output { f($(self.$numbers,)*) }
        }
        impl<Fun,Z,$($letters,)*> CanApplyMut<Fun> for ($($letters,)*) where Fun:FnMut($($letters,)*)->Z {
            fn apply_to_mut(self, f:&mut Fun) -> Self::Output { f($(self.$numbers,)*) }
        }
        impl<Fun,Z,$($letters,)*> CanApplyOnce<Fun> for ($($letters,)*) where Fun:FnOnce($($letters,)*)->Z {
            type Output = Z;
            fn apply_to_once(self, f:Fun) -> Self::Output { f($(self.$numbers,)*) }
        }
    };
}
mk_funcs!(ONE; []; []; []; []; []; []); // zero items
mk_funcs!(ONE; [A]; []; []; [0]; []; []); // one item
mk_funcs!(TWO; A ; [] ; B ; 0 ; [] ; 1);
mk_funcs!(TWO; A ; [B] ; C ; 0 ; [1] ; 2);
mk_funcs!(TWO; A ; [B,C] ; D ; 0 ; [1,2] ; 3);
mk_funcs!(TWO; A ; [B,C,D] ; E ; 0 ; [1,2,3] ; 4);
mk_funcs!(TWO; A ; [B,C,D,E] ; F ; 0 ; [1,2,3,4] ; 5);
mk_funcs!(TWO; A ; [B,C,D,E,F] ; G ; 0 ; [1,2,3,4,5] ; 6);
mk_funcs!(TWO; A ; [B,C,D,E,F,G] ; H ; 0 ; [1,2,3,4,5,6] ; 7);
mk_funcs!(TWO; A ; [B,C,D,E,F,G,H] ; I ; 0 ; [1,2,3,4,5,6,7] ; 8);
mk_funcs!(TWO; A ; [B,C,D,E,F,G,H,I] ; J ; 0 ; [1,2,3,4,5,6,7,8] ; 9);
mk_funcs!(TWO; A ; [B,C,D,E,F,G,H,I,J] ; K ; 0 ; [1,2,3,4,5,6,7,8,9] ; 10);
mk_funcs!(TWO; A ; [B,C,D,E,F,G,H,I,J,K] ; L ; 0 ; [1,2,3,4,5,6,7,8,9,10] ; 11);


#[cfg(test)]
mod tests {
    use super::*;

    fn do_push<W,X>(w:W, x:X) -> Push<W,X> where W:CanPush<X> { w.push(x) }
    fn do_pop<W>(w:W) -> Pop<W> where W:CanPop { w.pop() }
    fn do_shift<W>(w:W) -> Shift<W> where W:CanShift { w.shift() }
    fn do_unshift<W,X>(w:W, x:X) -> Unshift<W,X> where W:CanUnshift<X> { w.unshift(x) }

    fn do_shift_unshift<W,X>(w:W, x:X) -> Unshift<Shift<W>,X> where
        W:CanShift, Shift<W>:CanUnshift<X>
    {
        let w = w.shift();
        w.unshift(x)
    }

    fn replace_last_item<W,X>(w:W, x:X) -> Push<Pop<W>,X> where
        W:CanPop, Pop<W>:CanPush<X>
    {
        let w = w.pop();
        w.push(x)
    }

    use std::time::Instant;
    fn push_current_time<W>(w:W) -> Push<W,Instant> where W:CanPush<Instant> {
        w.push(Instant::now())
    }

    #[test]
    fn main_test() {
        println!("{:?}", do_push((3,&"hello"), 4.5));
        println!("{:?}", do_pop((3,&"hello")));
        println!("{:?}", do_shift((3,&"hello")));
        println!("{:?}", do_unshift((3,&"hello"), 3));

        let x = ();
        let x = do_push(x, 1);
        let x = do_push(x, 2);
        let x = do_push(x, 3);
        let x = do_unshift(x,0);
        let x = do_pop(x);
        let x = do_shift_unshift(x, 500);
        let x = do_shift(x);
        println!("(1,2) = {:?}", x);
        let x = replace_last_item(x, &"hi");
        println!("(1,\"hi\") = {:?}", x);

        let x = ();
        let x = do_unshift(x, 3);
        let x = do_unshift(x, 2);
        let x = do_unshift(x, 1);
        println!("(1,2,3) = {:?}", x);

        let x = push_current_time(x);
        println!("{:?}", x);

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
        x.apply_to(&|a,b|{ println!("{} {}", a, b); });


        let x = |a,b,c| {
            println!("{} / {} / {}", a, b, c);
        };

        let y = ();
        let y = y.push(1);
        let y = y.push(&"hello");
        let y = y.push(3.2);

        y.apply_to(&x);

        let x = ();
        let x = x.push(3);
        let x = push_current_time(x);
        let x = x.push(&"hi");
        let x = push_current_time(x);
        println!("{:?}", x);
    }
}