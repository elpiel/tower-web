use self::Source::*;

use http::header::HeaderName;

#[derive(Debug)]
pub struct CallSite {
    /// Where to extract the argument when the type does not provide the
    /// information.
    source: Source,
}

#[derive(Debug, Clone)]
pub(crate) enum Source {
    Capture(usize),
    Header(HeaderName),
    QueryString,
    Body,
    Unknown,
}

impl CallSite {
    pub fn new_capture(index: usize) -> CallSite {
        CallSite { source: Capture(index) }
    }

    pub fn new_header(name: &'static str) -> CallSite {
        CallSite { source: Header(HeaderName::from_static(name)) }
    }

    pub fn new_query_string() -> CallSite {
        CallSite { source: QueryString }
    }

    pub fn new_body() -> CallSite {
        CallSite { source: Body }
    }

    /// Cannot infer where to extract the argument based on the call site.
    pub fn new_unknown() -> CallSite {
        CallSite { source: Unknown }
    }

    pub(crate) fn source(&self) -> &Source {
        &self.source
    }

    pub(crate) fn requires_body(&self) -> bool {
        match self.source() {
            Body => true,
            _ => false,
        }
    }
}
