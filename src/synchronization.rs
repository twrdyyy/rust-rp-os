
use core::cell::UnsafeCell;


pub struct NullLock<T>
where T: ?Sized,
{
    data: UnsafeCell<T>,
}
/// Null Lock constructor
impl<T> NullLock<T> {
    
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}
unsafe impl<T> Send for NullLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for NullLock<T> where T: ?Sized + Sync {}

/// Interface used in synchronization
pub mod interface {
    /// Object implementing Mutex trait will be granted an exclusive access to data
    pub trait Mutex {
        
        type Data;

        fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R;
    }
}

impl<T> interface::Mutex for NullLock<T> {
    type Data = T;

    fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R {

        let data = unsafe { 
            &mut *self.data.get() 
        };

        f(data)
    }
}