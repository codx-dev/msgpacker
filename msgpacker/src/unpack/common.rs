use super::{
    helpers::{take_byte, take_byte_iter, take_num, take_num_iter},
    Error, Format, Unpackable,
};
use core::{marker::PhantomData, mem::MaybeUninit};
use std::ptr;

impl Unpackable for () {
    type Error = Error;

    fn unpack(_buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        Ok((0, ()))
    }

    fn unpack_iter<I>(_buf: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        Ok((0, ()))
    }
}

impl<X> Unpackable for PhantomData<X> {
    type Error = Error;

    fn unpack(_buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        Ok((0, PhantomData))
    }

    fn unpack_iter<I>(_buf: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        Ok((0, PhantomData))
    }
}

impl Unpackable for bool {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        if format != Format::TRUE && format != Format::FALSE {
            return Err(Error::UnexpectedFormatTag);
        }
        Ok((1, format != Format::FALSE))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        if format != Format::TRUE && format != Format::FALSE {
            return Err(Error::UnexpectedFormatTag);
        }
        Ok((1, format != Format::FALSE))
    }
}

impl<X> Unpackable for Option<X>
where
    X: Unpackable,
{
    type Error = <X as Unpackable>::Error;

    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        if buf.is_empty() {
            return Err(Error::BufferTooShort.into());
        }
        if buf[0] == Format::NIL {
            return Ok((1, None));
        }
        X::unpack(buf).map(|(n, x)| (n, Some(x)))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter().peekable();
        let format = *bytes.peek().ok_or(Error::BufferTooShort)?;
        if format == Format::NIL {
            bytes.next();
            return Ok((1, None));
        }
        X::unpack_iter(bytes).map(|(n, x)| (n, Some(x)))
    }
}

macro_rules! array {
    ($n:expr) => {
        impl<X> Unpackable for [X; $n]
        where
            X: Unpackable,
        {
            type Error = <X as Unpackable>::Error;

            fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
                let mut array = [const { MaybeUninit::uninit() }; $n];

                let format = take_byte(&mut buf)?;
                let (mut n, len) = match format {
                    0x90..=0x9f => (1, (format & 0x0f) as usize),
                    Format::ARRAY16 => (
                        3,
                        take_num(&mut buf, u16::from_be_bytes).map(|v| v as usize)?,
                    ),
                    Format::ARRAY32 => (
                        5,
                        take_num(&mut buf, u32::from_be_bytes).map(|v| v as usize)?,
                    ),
                    _ => return Err(Error::UnexpectedFormatTag.into()),
                };

                if len != $n {
                    return Err(Error::UnexpectedArrayLength.into());
                }

                n += array
                    .iter_mut()
                    .try_fold::<_, _, Result<_, Self::Error>>(0, |count, a| {
                        let (n, x) = X::unpack(buf)?;
                        buf = &buf[n..];
                        a.write(x);
                        Ok(count + n)
                    })?;
                // Safety: array is initialized
                let array = unsafe { ptr::read(&array as *const _ as *const [X; $n]) };
                Ok((n, array))
            }

            fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
            where
                I: IntoIterator<Item = u8>,
            {
                let mut bytes = bytes.into_iter();
                let mut array = [const { MaybeUninit::uninit() }; $n];

                let format = take_byte_iter(bytes.by_ref())?;
                let (mut n, len) = match format {
                    0x90..=0x9f => (1, (format & 0x0f) as usize),
                    Format::ARRAY16 => (
                        3,
                        take_num_iter(bytes.by_ref(), u16::from_be_bytes).map(|v| v as usize)?,
                    ),
                    Format::ARRAY32 => (
                        5,
                        take_num_iter(bytes.by_ref(), u32::from_be_bytes).map(|v| v as usize)?,
                    ),
                    _ => return Err(Error::UnexpectedFormatTag.into()),
                };

                if len != $n {
                    return Err(Error::UnexpectedArrayLength.into());
                }

                n += array
                    .iter_mut()
                    .try_fold::<_, _, Result<_, Self::Error>>(0, |count, a| {
                        let (n, x) = X::unpack_iter(bytes.by_ref())?;
                        a.write(x);
                        Ok(count + n)
                    })?;
                // Safety: array is initialized
                let array = unsafe { ptr::read(&array as *const _ as *const [X; $n]) };
                Ok((n, array))
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
    ($err:ident, $($name:ident)+) => (
        impl<$($name,)+> Unpackable for ($($name,)+)
        where
            $($name: Unpackable,)+
            $($err::Error: From<<$name as Unpackable>::Error>,)+
        {
            type Error = <$err as Unpackable>::Error;

            #[allow(non_snake_case)]
            fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
                let mut n = 0;

                $(let (c, $name) = $name::unpack(buf)?; n += c; buf = &buf[c..];)+
                let _ = buf;

                Ok((n, ($($name, )+)))
            }

            #[allow(non_snake_case)]
            fn unpack_iter<II>(bytes: II) -> Result<(usize, Self), Self::Error>
            where
                II: IntoIterator<Item = u8>,
            {
                let mut n = 0;
                let mut bytes = bytes.into_iter();

                $(let (c, $name) = $name::unpack_iter(bytes.by_ref())?; n += c;)+

                Ok((n, ($($name, )+)))
            }
        }
    );
}

tuple! {
    A, A
}
tuple! {
    A, A B
}
tuple! {
    A, A B C
}
tuple! {
    A, A B C D
}
tuple! {
    A, A B C D E
}
tuple! {
    A, A B C D E F
}
tuple! {
    A, A B C D E F G
}
tuple! {
    A, A B C D E F G H
}
tuple! {
    A, A B C D E F G H I
}
tuple! {
    A, A B C D E F G H I J
}
tuple! {
    A, A B C D E F G H I J K
}
tuple! {
    A, A B C D E F G H I J K L
}
tuple! {
    A, A B C D E F G H I J K L M
}
tuple! {
    A, A B C D E F G H I J K L M N
}
tuple! {
    A, A B C D E F G H I J K L M N O
}
tuple! {
    A, A B C D E F G H I J K L M N O P
}
tuple! {
    A, A B C D E F G H I J K L M N O P Q
}
tuple! {
    A, A B C D E F G H I J K L M N O P Q R
}
tuple! {
    A, A B C D E F G H I J K L M N O P Q R S
}
tuple! {
    A, A B C D E F G H I J K L M N O P Q R S T
}
tuple! {
    A, A B C D E F G H I J K L M N O P Q R S T U
}
tuple! {
    A, A B C D E F G H I J K L M N O P Q R S T U V
}
tuple! {
    A, A B C D E F G H I J K L M N O P Q R S T U V W
}
tuple! {
    A, A B C D E F G H I J K L M N O P Q R S T U V W X
}
tuple! {
    A, A B C D E F G H I J K L M N O P Q R S T U V W X Y
}
tuple! {
    A, A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
}
