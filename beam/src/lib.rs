#[cfg(test)]
mod tests;

use std::mem::MaybeUninit;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BeamError {
    BranchExhausted,
    Exhausted,
}

pub trait Node<const B: usize>
where
    Self: Sized,
{
    /// Heuristic function returning node's fulfillment status
    fn has_fulfilled(&self) -> bool;

    #[cfg(not(feature = "rayon"))]
    /// Generate successor nodes and write them to buffer
    fn expand<'a, I: Iterator<Item = &'a mut MaybeUninit<Self>>>(
        &'a self,
        iter: I,
    ) -> Result<usize, BeamError>;

    #[cfg(feature = "rayon")]
    /// Generate successor nodes and write them to buffer (parallel)
    fn expand<'a, I: IndexedParallelIterator<Item = &'a mut MaybeUninit<Self>>>(
        &'a self,
        iter: I,
    ) -> Result<usize, BeamError>;

    /// Node's score heuristics funciton for [`Beam::cycle`]
    fn evaluate(&self) -> u64;

    /// Mutate nodes in [`Beam::node_buf`] to meet criteria for [`Node::expand`]
    fn inflate(&mut self) {
        unimplemented!()
    }

    fn estimate(&self) -> Option<usize> {
        None
    }
}

#[derive(Debug)]
pub struct Beam<const W: usize, const B: usize, T>
where
    T: Node<B>,
{
    node_buf: Box<[MaybeUninit<T>]>,
    len: usize,
}

#[cfg(not(feature = "rayon"))]
impl<const W: usize, const B: usize, T> From<T> for Beam<W, B, T>
where
    T: Node<B> + Default,
{
    fn from(value: T) -> Self {
        #[inline]
        fn nodes_mut<T>(
            node_buf: &mut Box<[MaybeUninit<T>]>,
            len: usize,
        ) -> impl Iterator<Item = &mut MaybeUninit<T>> {
            node_buf
                .chunks_exact_mut(node_buf.len() / len)
                .map(|c| c.first_mut().unwrap())
        }

        let len = value.estimate().unwrap_or(W);

        let mut node_buf = Box::new_uninit_slice((len + 1) * W);
        let count = value.expand(nodes_mut(&mut node_buf, len)).unwrap();
        assert_ne!(count, 0);

        // fill leftover nodes
        nodes_mut(&mut node_buf, len).skip(count).for_each(|node| {
            let _ = node.write(T::default());
        });

        Self { node_buf, len }
    }
}

#[cfg(feature = "rayon")]
impl<const W: usize, const B: usize, T> From<T> for Beam<W, B, T>
where
    T: Node<B> + Default + Send,
{
    fn from(value: T) -> Self {
        #[inline]
        fn nodes_mut<T: Send>(
            node_buf: &mut Box<[MaybeUninit<T>]>,
            len: usize,
        ) -> impl IndexedParallelIterator<Item = &mut MaybeUninit<T>> {
            node_buf
                .par_chunks_exact_mut(node_buf.len() / len)
                .map(|c| c.first_mut().unwrap())
        }

        let len = value.estimate().unwrap_or(W);

        let mut node_buf = Box::new_uninit_slice((len + 1) * W);
        let count = value.expand(nodes_mut(&mut node_buf, len)).unwrap();
        assert_ne!(count, 0);

        // fill leftover nodes
        nodes_mut(&mut node_buf, len).skip(count).for_each(|node| {
            let _ = node.write(T::default());
        });

        Self { node_buf, len }
    }
}

impl<const W: usize, const B: usize, T> Beam<W, B, T>
where
    T: Node<B>,
{
    #[cfg(not(feature = "rayon"))]
    #[inline]
    pub fn has_fulfilled(&self) -> bool {
        self.nodes().any(|n| n.has_fulfilled())
    }

    #[cfg(feature = "rayon")]
    #[inline]
    pub fn has_fulfilled(&self) -> bool
    where
        T: Sync,
    {
        self.nodes().any(|n| n.has_fulfilled())
    }

    #[cfg(not(feature = "rayon"))]
    #[inline]
    pub fn nodes(&self) -> impl Iterator<Item = &T> {
        self.node_buf
            .chunks_exact(self.node_buf.len() / self.len)
            .map(|c| unsafe { c.first().unwrap_unchecked().assume_init_ref() })
            .filter(|c| c.has_fulfilled())
    }

    #[cfg(feature = "rayon")]
    #[inline]
    pub fn nodes(&self) -> impl ParallelIterator<Item = &T>
    where
        T: Sync,
    {
        self.node_buf
            .par_chunks_exact(self.node_buf.len() / self.len)
            .map(|c| unsafe { c.first().unwrap_unchecked().assume_init_ref() })
            .filter(|c| c.has_fulfilled())
    }

    #[cfg(not(feature = "rayon"))]
    #[inline]
    fn nodes_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.node_buf
            .chunks_exact_mut(self.node_buf.len() / self.len)
            .map(|c| unsafe { c.first_mut().unwrap_unchecked().assume_init_mut() })
    }

    #[cfg(feature = "rayon")]
    #[inline]
    fn nodes_mut(&mut self) -> impl IndexedParallelIterator<Item = &mut T>
    where
        T: Send,
    {
        self.node_buf
            .par_chunks_exact_mut(self.node_buf.len() / self.len)
            .map(|c| unsafe { c.first_mut().unwrap_unchecked().assume_init_mut() })
    }

    #[cfg(not(feature = "rayon"))]
    #[inline]
    fn split_mut(&mut self) -> impl Iterator<Item = (&mut T, &mut [MaybeUninit<T>])> {
        self.node_buf
            .chunks_exact_mut(self.node_buf.len() / self.len)
            .map(|c| unsafe {
                let (l, r) = c.split_first_mut().unwrap_unchecked();
                (l.assume_init_mut(), r)
            })
    }

    #[cfg(feature = "rayon")]
    #[inline]
    fn split_mut(&mut self) -> impl IndexedParallelIterator<Item = (&mut T, &mut [MaybeUninit<T>])>
    where
        T: Send,
    {
        self.node_buf
            .par_chunks_exact_mut(self.node_buf.len() / self.len)
            .map(|c| unsafe {
                let (l, r) = c.split_first_mut().unwrap_unchecked();
                (l.assume_init_mut(), r)
            })
    }

    #[cfg(not(feature = "rayon"))]
    pub fn cycle(&mut self) -> Result<(), BeamError> {
        let cond = self
            .split_mut()
            .map(|(node, buf)| -> Result<(), BeamError> {
                // expansion
                let i = node.expand(buf.iter_mut())?;

                // evaluation + selection
                *node = unsafe {
                    buf.iter()
                        .take(i)
                        .map(|x| x.assume_init_read()) // dropped
                        .min_by_key(|k| k.evaluate())
                        .unwrap_unchecked()
                };

                Ok(())
            })
            .fold(true, |acc, c| {
                acc & matches!(c, Err(BeamError::BranchExhausted))
            });

        std::hint::select_unpredictable(cond, Err(BeamError::Exhausted), Ok(()))
    }

    #[cfg(feature = "rayon")]
    pub fn cycle(&mut self) -> Result<(), BeamError>
    where
        T: Send,
    {
        let cond = self
            .split_mut()
            .map(|(node, buf)| -> Result<(), BeamError> {
                // expansion
                let i = node.expand(buf.par_iter_mut())?;

                // evaluation + selection
                *node = unsafe {
                    buf.par_iter_mut()
                        .take(i)
                        .map(|x| x.assume_init_read()) // dropped
                        .min_by_key(|k| k.evaluate())
                        .unwrap_unchecked()
                };

                Ok(())
            })
            .fold_with(true, |acc, c| {
                acc && matches!(c, Err(BeamError::BranchExhausted))
            })
            .reduce(|| true, |acc, c| acc && c);

        std::hint::select_unpredictable(cond, Err(BeamError::Exhausted), Ok(()))
    }

    #[cfg(not(feature = "rayon"))]
    #[inline]
    pub fn extend(&mut self) {
        self.nodes_mut().for_each(|node| {
            node.inflate();
        });
    }

    #[cfg(feature = "rayon")]
    #[inline]
    pub fn extend(&mut self)
    where
        T: Send,
    {
        self.nodes_mut().for_each(|node| {
            node.inflate();
        });
    }
}
