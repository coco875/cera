#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TextPosition {
    /// Points to the bit idx of the first char this position represents
    pub idx: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TextSpan {
    pub len: usize,
    pub idx: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Text<'t> {
    inner: &'t str,
    /// Sorted from smallest to largest
    /// These indices must be within "inner"
    line_feed_indices: Box<[usize]>,
}

impl<'t> Text<'t> {
    pub fn new(str: &'t str) -> Self {
        let indices: Vec<_> = str
            .char_indices()
            .filter_map(|(idx, char)| if char == '\n' { Some(idx) } else { None })
            .collect();
        Self {
            inner: str,
            line_feed_indices: indices.into_boxed_slice(),
        }
    }
    /// Returns in which line the given idx is, starting at 0
    pub fn find_line(&self, idx: usize) -> usize {
        match self.line_feed_indices.binary_search(&idx) {
            Ok(val) => val,
            Err(val) => val,
        }
    }
    pub fn inner(&self) -> &'t str {
        self.inner
    }
    /// Returns the line at the index, line feed included, 0-indexed
    #[inline]
    pub fn line(&self, line: usize) -> Option<&'t str> {
        if line == 0 {
            return self.inner.get(0..(*self.line_feed_indices.first()?));
        }
        let start = *self.line_feed_indices.get(line - 1)?;
        let end = self
            .line_feed_indices
            .get(line)
            .map(|idx| idx + 1)
            .unwrap_or_else(|| self.inner.len());
        self.inner.get(start..end).filter(|str| !str.is_empty())
    }

    pub fn lines(&'t self) -> Lines<'t> {
        Lines {
            idx: 0,
            inner: self,
        }
    }
}

pub struct Lines<'t> {
    idx: usize,
    inner: &'t Text<'t>,
}

impl<'t> Iterator for Lines<'t> {
    type Item = &'t str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.inner.line(self.idx);
        self.idx += 1;
        ret
    }
}
