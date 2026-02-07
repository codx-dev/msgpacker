use super::{
    helpers::{take_byte, take_byte_iter},
    Error, Format, Unpackable,
};
use core::{marker::PhantomData, mem::MaybeUninit};

impl Unpackable for () {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        if format != Format::NIL {
            return Err(Error::UnexpectedFormatTag);
        }
        Ok((1, ()))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        if format != Format::NIL {
            return Err(Error::UnexpectedFormatTag);
        }
        Ok((1, ()))
    }
}

impl<X> Unpackable for PhantomData<X> {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        if format != Format::NIL {
            return Err(Error::UnexpectedFormatTag);
        }
        Ok((1, PhantomData))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        if format != Format::NIL {
            return Err(Error::UnexpectedFormatTag);
        }
        Ok((1, PhantomData))
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

impl Unpackable for char {
    type Error = Error;

    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        u32::unpack(buf)
            .and_then(|(n, v)| char::from_u32(v).ok_or(Error::InvalidUtf8).map(|c| (n, c)))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        u32::unpack_iter(bytes)
            .and_then(|(n, v)| char::from_u32(v).ok_or(Error::InvalidUtf8).map(|c| (n, c)))
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

impl<X, const N: usize> Unpackable for [X; N]
where
    X: Unpackable,
{
    type Error = <X as Unpackable>::Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let mut array = [const { MaybeUninit::uninit() }; N];
        let n = array
            .iter_mut()
            .try_fold::<_, _, Result<_, Self::Error>>(0, |count, a| {
                let (n, x) = X::unpack(buf)?;
                buf = &buf[n..];
                a.write(x);
                Ok(count + n)
            })?;
        // Safety: array is initialized
        let array = ::core::array::from_fn(|i| {
            let mut x = MaybeUninit::zeroed();
            ::core::mem::swap(&mut array[i], &mut x);
            unsafe { MaybeUninit::assume_init(x) }
        });
        Ok((n, array))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let mut array = [const { MaybeUninit::uninit() }; N];
        let n = array
            .iter_mut()
            .try_fold::<_, _, Result<_, Self::Error>>(0, |count, a| {
                let (n, x) = X::unpack_iter(bytes.by_ref())?;
                a.write(x);
                Ok(count + n)
            })?;
        // Safety: array is initialized
        let array = ::core::array::from_fn(|i| {
            let mut x = MaybeUninit::zeroed();
            ::core::mem::swap(&mut array[i], &mut x);
            unsafe { MaybeUninit::assume_init(x) }
        });
        Ok((n, array))
    }
}

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
