/// Offset trait I took from fantastic parser combinator library https://github.com/Geal/nom
/// which is under MIT license.

pub trait Offset {
    fn offset(&self, second: &Self) -> usize;
}

impl Offset for str {
    fn offset(&self, second: &Self) -> usize {
        let fst = self.as_ptr();
        let snd = second.as_ptr();

        snd as usize - fst as usize
    }
}

impl<'a> Offset for &'a str {
    fn offset(&self, second: &Self) -> usize {
        let fst = self.as_ptr();
        let snd = second.as_ptr();

        snd as usize - fst as usize
    }
}
