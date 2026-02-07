use super::{Format, Packable};
use core::{iter, marker::PhantomData};

impl Packable for () {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        buf.extend(iter::once(Format::NIL));
        1
    }
}

impl<X> Packable for PhantomData<X> {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        buf.extend(iter::once(Format::NIL));
        1
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

impl Packable for char {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        (*self as u32).pack(buf)
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

impl<X, const N: usize> Packable for [X; N]
where
    X: Packable,
{
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        self.iter().map(|t| t.pack(buf)).sum()
    }
}

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
