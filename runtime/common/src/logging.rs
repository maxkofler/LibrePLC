#[allow(unused_macros)]
macro_rules! info{
    ($($arg:tt)+) => {
        #[cfg(feature = "logging_log")]
        log::info!($($arg)+)
    }
}

#[allow(unused_macros)]
macro_rules! debug{
    ($($arg:tt)+) => {
        #[cfg(feature = "logging_log")]
        log::debug!($($arg)+)
    }
}

#[allow(unused_macros)]
macro_rules! trace{
    ($($arg:tt)+) => {
        #[cfg(feature = "logging_log")]
        log::trace!($($arg)+)
    }
}

#[allow(unused_imports)]
pub(crate) use {debug, info, trace};
