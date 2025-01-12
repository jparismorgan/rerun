use std::sync::mpsc;

/// Creates a new context.
///
/// The [`Reporter`] can be freely cloned and sent to other threads.
///
/// The [`Report`] should not be sent to other threads.
pub fn init() -> (Report, Reporter) {
    let (tx, rx) = mpsc::channel();
    (Report::new(rx), Reporter::new(tx))
}

/// Used to accumulate errors and warnings.
#[derive(Clone)]
pub struct Reporter {
    errors: mpsc::Sender<anyhow::Error>,
}

impl Reporter {
    fn new(errors: mpsc::Sender<anyhow::Error>) -> Self {
        Self { errors }
    }

    pub fn error(&self, error: impl IntoError) {
        let _ = self.errors.send(error.into_error());
    }
}

/// Report which holds accumulated errors and warnings.
///
/// This should only exist on the main thread.
pub struct Report {
    errors: mpsc::Receiver<anyhow::Error>,
    _not_send: std::marker::PhantomData<*mut ()>,
}

impl Report {
    fn new(errors: mpsc::Receiver<anyhow::Error>) -> Self {
        Self {
            errors,
            _not_send: std::marker::PhantomData,
        }
    }

    /// This outputs all errors to stderr and panics if there were any.
    pub fn panic_on_errors(&self) {
        let mut errored = false;

        while let Ok(err) = self.errors.try_recv() {
            errored = true;
            eprintln!("{err}");
        }

        #[allow(clippy::manual_assert)] // we don't want the noise of an assert
        if errored {
            panic!("Some errors occurred.");
        }
    }
}

const _: () = {
    // We want to ensure `Report` is `!Send`, so that it stays
    // on the main thread.
    //
    // This works by creating a type which has a different number of possible
    // implementations of a trait depending on its `Send`-ness:
    // - One impl for all `T: !Send`
    // - Two impls for all `T: Send`
    //
    // In an invocation like `Check<T> as IsNotSend<_>`, we're asking
    // the compiler to infer the type given to `IsNotSend` for us.
    // But if `Check<T>: Send`, then it has two possible implementations:
    // `IsNotSend<True>`, and `IsNotSend<False>`. This is a local ambiguity
    // and rustc has no way to disambiguate between the two implementations,
    // so it will instead output a type inference error.
    //
    // We could use `static_assertions` here, but we'd rather not pay the
    // compilation cost for a single invocation. Some of the macros from
    // that crate are pretty gnarly!
    trait IsNotSend<T> {
        fn __() {}
    }

    type False = ();

    struct True;

    struct Check<T: ?Sized>(T);

    impl<T: ?Sized> IsNotSend<True> for Check<T> {}

    impl<T: ?Sized + Send> IsNotSend<False> for Check<T> {}

    // if this fails with a type inference error,
    // then `Report` is `Send`, which it should _not_ be.
    let _ = <Check<Report> as IsNotSend<_>>::__;

    fn assert_send<T: Send>() {}
    let _ = assert_send::<Reporter>;
};

#[derive(Debug)]
struct Error {
    message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}

pub trait IntoError {
    fn into_error(self) -> anyhow::Error;
}

impl IntoError for String {
    fn into_error(self) -> anyhow::Error {
        Error { message: self }.into()
    }
}

impl<'a> IntoError for &'a str {
    fn into_error(self) -> anyhow::Error {
        Error {
            message: self.into(),
        }
        .into()
    }
}

impl<'a> IntoError for std::borrow::Cow<'a, str> {
    fn into_error(self) -> anyhow::Error {
        Error {
            message: self.into(),
        }
        .into()
    }
}

impl IntoError for anyhow::Error {
    fn into_error(self) -> anyhow::Error {
        self
    }
}
