use std::{
    fmt::Debug,
    mem,
    ops::{Deref, DerefMut},
    ptr,
};

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DetectDrop<T: Debug + ?Sized>(pub T);

impl<T: Debug> DetectDrop<T> {
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    pub fn into_inner(self) -> T {
        unsafe {
            let inner = ptr::read(&self as *const Self as *const T);
            mem::forget(self);
            inner
        }
    }
}

impl<T: Debug + ?Sized> Deref for DetectDrop<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Debug + ?Sized> DerefMut for DetectDrop<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Debug + ?Sized> Drop for DetectDrop<T> {
    fn drop(&mut self) {
        println!("dropping {:?}", &self.0);
    }
}

impl<T: Debug> From<T> for DetectDrop<T> {
    fn from(value: T) -> Self {
        DetectDrop(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = 10;
        let _ = DetectDrop::new(a + 3);
    }

    #[test]
    fn drop_vec() {
        let v = vec![
            DetectDrop::new("abc".to_owned()),
            DetectDrop::new("defg".to_owned()),
            DetectDrop::new("hijkl".to_owned()),
        ];
        drop(v);
    }
}
