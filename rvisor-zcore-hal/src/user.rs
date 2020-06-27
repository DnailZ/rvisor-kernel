use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use core::mem::size_of;
use lkm::user_ptr::{
    UserSlicePtr,
    readstr_from_user,
    writestr_to_user,
};
#[repr(C)]
pub struct UserPtr<T, P: Policy> {
    ptr: *mut T,
    mark: PhantomData<P>,
}

pub trait Policy {}
pub trait Read: Policy {}
pub trait Write: Policy {}
pub enum In {}
pub enum Out {}
pub enum InOut {}

impl Policy for In {}
impl Policy for Out {}
impl Policy for InOut {}
impl Read for In {}
impl Write for Out {}
impl Read for InOut {}
impl Write for InOut {}

pub type UserInPtr<T> = UserPtr<T, In>;
pub type UserOutPtr<T> = UserPtr<T, Out>;
pub type UserInOutPtr<T> = UserPtr<T, InOut>;

type Result<T> = core::result::Result<T, Error>;

/// The error type which is returned from user pointer.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidUtf8,
    InvalidPointer,
    BufferTooSmall,
}

impl<T, P: Policy> Debug for UserPtr<T, P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.ptr)
    }
}

// FIXME: this is a workaround for `clear_child_tid`.
unsafe impl<T, P: Policy> Send for UserPtr<T, P> {}
unsafe impl<T, P: Policy> Sync for UserPtr<T, P> {}

impl<T, P: Policy> From<usize> for UserPtr<T, P> {
    fn from(x: usize) -> Self {
        UserPtr {
            ptr: x as _,
            mark: PhantomData,
        }
    }
}

impl<T, P: Policy> UserPtr<T, P> {
    pub fn from_addr_size(addr: usize, size: usize) -> Result<Self> {
        if size < core::mem::size_of::<T>() {
            return Err(Error::BufferTooSmall);
        }
        Ok(Self::from(addr))
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
    
    pub fn add(&self, count: usize) -> Self {
        warn!("UserPtr added!");
        UserPtr {
            ptr: unsafe { self.ptr.add(count) },
            mark: PhantomData,
        }
    }
    
    pub fn as_ptr(&self) -> *mut T {
        warn!("UserPtr as_ptr!");
        self.ptr
    }

    pub fn check(&self) -> Result<()> {
        if self.ptr.is_null() {
            return Err(Error::InvalidPointer);
        }
        Ok(())
    }
}

impl<T, P: Read> UserPtr<T, P> {
    pub fn as_ref(&self) -> Result<&'static T> {
        warn!("UserPtr as_ref!");
        Ok(unsafe { &*self.ptr })
    }

    // ! modified
    pub fn read(&self) -> Result<T> {
        trace!("UserPtr::read");
        
        // TODO: check ptr and return err
        self.check()?;

        let mut data = UserSlicePtr::new_ptr(self.ptr as u64, size_of::<T>())
            .map_err(|_| { Error::InvalidPointer })?
            .read_all()
            .map_err(|_| { Error::InvalidPointer })?;
        let data = data.as_mut_ptr() as *mut T;

        Ok(unsafe {
            data.read()
        })
    }

    pub fn read_if_not_null(&self) -> Result<Option<T>> {
        if self.ptr.is_null() {
            return Ok(None);
        }
        let value = self.read()?;
        Ok(Some(value))
    }

    // ! modified
    pub fn read_array(&self, len: usize) -> Result<Vec<T>> {
        trace!("UserPtr::read_array");
        if len == 0 {
            return Ok(Vec::default());
        }
        self.check()?;

        let mut data = Vec::<T>::with_capacity(len);

        UserSlicePtr::new_ptr(self.ptr as u64, size_of::<T>()*len)
            .map_err(|_| Error::InvalidPointer)?
            .reader()
            .read_mut_slice(data.as_mut_slice())
            .map_err(|_| Error::InvalidPointer)?;
            
        unsafe { data.set_len(len);}

        Ok(data)
    }
}

impl<P: Read> UserPtr<u8, P> {
    // ! modified
    pub fn read_string(&self, len: usize) -> Result<String> {
        // assume len is 
        trace!("UserPtr::read_string");
        self.check()?;

        let mut data = String::with_capacity(len);
        UserSlicePtr::new_ptr(self.ptr as u64, size_of::<u8>() * len)
            .map_err(|_| Error::InvalidPointer)?
            .reader()
            .read_mut_slice(data.as_bytes_mut())
            .map_err(|_| Error::InvalidPointer)?;
        Ok(String::from(s))
    }

    pub fn read_cstring(&self) -> Result<String> {
        trace!("UserPtr::read_cstring");
        self.check()?;
        let len = unsafe { (0usize..).find(|&i| *self.ptr.add(i) == 0).unwrap() };
        self.read_string(len)
    }
}

impl<P: Read> UserPtr<UserPtr<u8, P>, P> {
    pub fn read_cstring_array(&self) -> Result<Vec<String>> {
        trace!("UserPtr::read_cstring_array");
        self.check()?;
        let len = unsafe {
            (0usize..)
                .find(|&i| self.ptr.add(i).read().is_null())
                .unwrap()
        };
        self.read_array(len)?
            .into_iter()
            .map(|ptr| ptr.read_cstring())
            .collect()
    }
}

impl<T, P: Write> UserPtr<T, P> {
    pub fn write(&mut self, value: T) -> Result<()> {
        trace!("UserPtr::write");
        self.check()?;
        unsafe {
            self.ptr.write(value);
        }
        Ok(())
    }

    pub fn write_if_not_null(&mut self, value: T) -> Result<()> {
        trace!("UserPtr::write_if_not_null");
        if self.ptr.is_null() {
            return Ok(());
        }
        self.write(value)
    }

    pub fn write_array(&mut self, values: &[T]) -> Result<()> {
        trace!("UserPtr::write_array");
        if values.is_empty() {
            return Ok(());
        }
        self.check()?;
        unsafe {
            self.ptr
                .copy_from_nonoverlapping(values.as_ptr(), values.len());
        }
        Ok(())
    }
}

impl<P: Write> UserPtr<u8, P> {
    pub fn write_cstring(&mut self, s: &str) -> Result<()> {
        trace!("UserPtr::write_cstring");
        let bytes = s.as_bytes();
        self.write_array(bytes)?;
        unsafe {
            self.ptr.add(bytes.len()).write(0);
        }
        Ok(())
    }
}
