use super::{get_array_info, Format, Packable};
use core::{iter, marker::PhantomData};

impl Packable for () {
    fn pack<T>(&self, _buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        0
    }
}

impl<X> Packable for PhantomData<X> {
    fn pack<T>(&self, _buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        0
    }
}

impl Packable for bool {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self {
            buf.extend(iter::once(Format::TRUE));
        } else {
            buf.extend(iter::once(Format::FALSE));
        }
        1
    }
}

impl<X> Packable for Option<X>
where
    X: Packable,
{
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        match self {
            Some(t) => t.pack(buf),
            None => {
                buf.extend(iter::once(Format::NIL));
                1
            }
        }
    }
}

macro_rules! array {
    ($n:expr) => {
        impl<X> Packable for [X; $n]
        where
            X: Packable,
        {
            fn pack<T>(&self, buf: &mut T) -> usize
            where
                T: Extend<u8>,
            {
                let len = self.len();
                let n = get_array_info(buf, len);
                n + self.iter().map(|t| t.pack(buf)).sum::<usize>()
            }
        }
    };
}

array!(0);
array!(1);
array!(2);
array!(3);
array!(4);
array!(5);
array!(6);
array!(7);
array!(8);
array!(9);
array!(10);
array!(11);
array!(12);
array!(13);
array!(14);
array!(15);
array!(16);
array!(17);
array!(18);
array!(19);
array!(20);
array!(21);
array!(22);
array!(23);
array!(24);
array!(25);
array!(26);
array!(27);
array!(28);
array!(29);
array!(30);
array!(31);
array!(32);
array!(33);
array!(34);
array!(35);
array!(36);
array!(37);
array!(38);
array!(39);
array!(40);
array!(41);
array!(42);
array!(43);
array!(44);
array!(45);
array!(46);
array!(47);
array!(48);
array!(49);
array!(50);
array!(51);
array!(52);
array!(53);
array!(54);
array!(55);
array!(56);
array!(57);
array!(58);
array!(59);
array!(60);
array!(61);
array!(62);
array!(63);
array!(64);

macro_rules! tuple {
    ( $($name:ident)+) => (
        impl<$($name,)+> Packable for ($($name,)+)
        where $($name: Packable,)+
        {
            #[allow(non_snake_case)]
            fn pack<TT>(&self, buf: &mut TT) -> usize
            where
                TT: Extend<u8>,
            {
                let ($(ref $name,)+) = *self;

                0 $( + $name.pack(buf))+
            }
        }
    );
}

tuple! {
    A
}
tuple! {
    A B
}
tuple! {
    A B C
}
tuple! {
    A B C D
}
tuple! {
    A B C D E
}
tuple! {
    A B C D E F
}
tuple! {
    A B C D E F G
}
tuple! {
    A B C D E F G H
}
tuple! {
    A B C D E F G H I
}
tuple! {
    A B C D E F G H I J
}
tuple! {
    A B C D E F G H I J K
}
tuple! {
    A B C D E F G H I J K L
}
tuple! {
    A B C D E F G H I J K L M
}
tuple! {
    A B C D E F G H I J K L M N
}
tuple! {
    A B C D E F G H I J K L M N O
}
tuple! {
    A B C D E F G H I J K L M N O P
}
tuple! {
    A B C D E F G H I J K L M N O P Q
}
tuple! {
    A B C D E F G H I J K L M N O P Q R
}
tuple! {
    A B C D E F G H I J K L M N O P Q R S
}
tuple! {
    A B C D E F G H I J K L M N O P Q R S T
}
tuple! {
    A B C D E F G H I J K L M N O P Q R S T U
}
tuple! {
    A B C D E F G H I J K L M N O P Q R S T U V
}
tuple! {
    A B C D E F G H I J K L M N O P Q R S T U V W
}
tuple! {
    A B C D E F G H I J K L M N O P Q R S T U V W X
}
tuple! {
    A B C D E F G H I J K L M N O P Q R S T U V W X Y
}
tuple! {
    A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
}
