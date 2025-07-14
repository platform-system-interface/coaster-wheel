use crate::coaster::Coaster;

#[derive(Clone, Copy, Debug)]
pub struct Wheel<'a, const S: usize> {
    coaster: Coaster<'a, S>,
}

/// The Wheel, it turns, it comes around.
/// It makes an ancient rumbling sound.
/// <https://www.youtube.com/watch?v=fJ4o0eLKQlA>
impl<'a, const S: usize> Wheel<'a, S> {
    pub fn new(coaster: Coaster<'a, S>) -> Self {
        Self { coaster }
    }

    pub fn turn(&mut self) -> Option<usize> {
        self.coaster.step()
    }

    pub fn rotate(&mut self) {
        self.coaster.run();
    }

    pub fn schedule(self) {
        // TODO: turn or rotate?
    }
}
