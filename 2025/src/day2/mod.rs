use miette::{Diagnostic, NamedSource, Result, SourceSpan};
use std::fmt;

pub(crate) fn run() -> Result<()> {
    let src = "value = woops\nnext = ok".to_string();
    let src2 = "kek\norange\nnasty\nwoops\nfrench\npopulus\n".to_string();

    // Simulate detecting an error in "woops"
    let offset = src.find("woops").unwrap();
    let span: SourceSpan = (offset, 5).into();

    let offset2 = src2.find("woops").unwrap();
    let span2: SourceSpan = (offset2, 5).into();

    Err(MyError {
        message: "Invalid value detected".to_owned(),
        src: NamedSource::new("config.txt", src),
        span,
        cause: Some(Box::new(MyError {
            message: "On the blacklist".to_owned(),
            src: NamedSource::new("opt/blacklist.json", src2),
            span: span2,
            cause: None,
        })),
    }
    .into())
}

#[derive(Debug)]
struct MyError {
    message: String,
    src: NamedSource<String>,
    span: SourceSpan,
    cause: Option<Box<dyn Diagnostic + Send + Sync>>,
}

// ---- Implement Error + Display ----

impl std::error::Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// ---- Implement Diagnostic manually ----

impl Diagnostic for MyError {
    // The main source file
    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        Some(&self.src)
    }

    // Highlight the span
    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        let label =
            miette::LabeledSpan::new_with_span(Some("offending value".to_owned()), self.span);
        let label2 = miette::LabeledSpan::new_with_span(
            Some("Fuck this".to_owned()),
            SourceSpan::new(6.into(), 1),
        );

        let label3 = miette::LabeledSpan::new(None, 0, 5);
        Some(Box::new([label, label2, label3].into_iter()))
    }

    // A little help text
    fn help(&self) -> Option<Box<dyn std::fmt::Display + 'static>> {
        Some(Box::new("Try correcting the value."))
    }

    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        Some(Box::new("E122"))
    }

    fn severity(&self) -> Option<miette::Severity> {
        Some(miette::Severity::Advice)
    }

    fn url<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        Some(Box::new("https://adventofcode.com/docs/E122"))
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        match self.cause {
            Some(_) => {
                let kek = self.cause.as_deref().map(|c| c as &dyn Diagnostic).unwrap();
                Some(Box::new([kek].into_iter()))
            }
            None => None,
        }
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        // self.cause.as_deref().map(|c| c as &dyn Diagnostic)
        None
    }
}
