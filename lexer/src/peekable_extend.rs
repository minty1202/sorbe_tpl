use std::iter::Peekable;

pub trait PeekableExt<E> {
    fn read_until_delimiter<F, G>(
        &mut self,
        predicate: F,
        error_predicate: G,
    ) -> Result<String, char>
    where
        F: Fn(char) -> bool,
        G: Fn(char) -> Option<bool>;

    fn read_until_terminator<F, G>(
        &mut self,
        predicate: F,
        error_predicate: G,
    ) -> Result<String, char>
    where
        F: Fn(char) -> bool,
        G: Fn(char) -> Option<bool>;
}

impl<I> PeekableExt<char> for Peekable<I>
where
    I: Iterator<Item = char>,
{
    fn read_until_delimiter<F, G>(
        &mut self,
        predicate: F,
        error_predicate: G,
    ) -> Result<String, char>
    where
        F: Fn(char) -> bool,
        G: Fn(char) -> Option<bool>,
    {
        let mut result = String::new();

        loop {
            match self.peek() {
                Some(&c) => {
                    if let Some(true) = error_predicate(c) {
                        return Err(c);
                    }

                    if predicate(c) {
                        return Ok(result);
                    }

                    let actual_c = self
                        .next()
                        .expect("Iterator inconsistency: peek() succeeded but next() failed");
                    result.push(actual_c);
                }
                None => {
                    return Ok(result);
                }
            }
        }
    }

    fn read_until_terminator<F, G>(
        &mut self,
        predicate: F,
        error_predicate: G,
    ) -> Result<String, char>
    where
        F: Fn(char) -> bool,
        G: Fn(char) -> Option<bool>,
    {
        let mut result = String::new();

        loop {
            match self.peek() {
                Some(&c) => {
                    if let Some(true) = error_predicate(c) {
                        return Err(c);
                    }

                    if predicate(c) {
                        self.next();
                        return Ok(result);
                    }

                    let actual_c = self
                        .next()
                        .expect("Iterator inconsistency: peek() succeeded but next() failed");
                    result.push(actual_c);
                }
                None => {
                    return Err('\0');
                }
            }
        }
    }
}
