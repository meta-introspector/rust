use std::borrow::Cow;

macro_rules! cvs {
    () => {
        ::std::borrow::Cow::Borrowed(&[])
    };
    ($($x:expr),+ $(,)?) => {
        ::std::borrow::Cow::Borrowed(&[
            $(
                ::std::borrow::Cow::Borrowed($x),
            )*
        ])
    };
}

pub(crate) use cvs;