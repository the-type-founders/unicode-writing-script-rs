//! Detection of writing scripts from Unicode codepoints.

include!(concat!(env!("OUT_DIR"), "/data.rs"));

/// A match.
#[derive(Clone, Copy)]
pub struct Match {
    pub name: &'static str,
    pub count: usize,
    pub score: f64,
}

/// Detect writing scripts given ranges of Unicode codepoints.
pub fn detect<T>(_codepoints: T, _threshold: f64) -> Vec<Match>
where
    T: IntoIterator<Item = [u32; 2]>,
{
    let _ = [0; SCRIPT_COUNT];

    unimplemented!()
}
