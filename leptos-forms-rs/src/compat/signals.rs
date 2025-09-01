//! Signal compatibility layer for Leptos 0.6 and 0.8
//! 
//! Provides unified signal APIs that work with both versions.

use std::marker::PhantomData;

/// Version-agnostic signal wrapper
pub struct SignalCompat<T: Clone + 'static> {
    #[cfg(feature = "leptos-0-6")]
    read: leptos_06::ReadSignal<T>,
    #[cfg(feature = "leptos-0-6")]
    write: leptos_06::WriteSignal<T>,
    
    #[cfg(feature = "leptos-0-8")]
    read: leptos_08::prelude::ReadSignal<T>,
    #[cfg(feature = "leptos-0-8")]
    write: leptos_08::prelude::WriteSignal<T>,
    
    _phantom: PhantomData<T>,
}

impl<T: Clone + 'static> SignalCompat<T> {
    /// Create a new signal with initial value
    pub fn new(initial: T) -> Self {
        #[cfg(feature = "leptos-0-6")]
        {
            let (read, write) = leptos_06::create_signal(initial);
            Self { 
                read, 
                write, 
                _phantom: PhantomData 
            }
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            let (read, write) = leptos_08::prelude::signal(initial);
            Self { 
                read, 
                write, 
                _phantom: PhantomData 
            }
        }
    }
    
    /// Get the current signal value
    pub fn get(&self) -> T {
        #[cfg(feature = "leptos-0-6")]
        {
            self.read.get()
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            self.read.get()
        }
    }
    
    /// Set the signal value
    pub fn set(&self, value: T) {
        #[cfg(feature = "leptos-0-6")]
        {
            self.write.set(value)
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            self.write.set(value)
        }
    }
    
    /// Update the signal value using a closure
    pub fn update<F>(&self, f: F) 
    where F: FnOnce(&mut T) {
        #[cfg(feature = "leptos-0-6")]
        {
            self.write.update(f);
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            self.write.update(f);
        }
    }
    
    /// Get a reference to the read signal
    pub fn read_signal(&self) -> ReadSignalCompat<T> {
        #[cfg(feature = "leptos-0-6")]
        {
            ReadSignalCompat::new(self.read.clone())
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            ReadSignalCompat::new(self.read.clone())
        }
    }
    
    /// Get a reference to the write signal
    pub fn write_signal(&self) -> WriteSignalCompat<T> {
        #[cfg(feature = "leptos-0-8")]
        {
            WriteSignalCompat::new(self.write.clone())
        }
        
        #[cfg(feature = "leptos-0-6")]
        {
            WriteSignalCompat::new(self.write.clone())
        }
    }
}

impl<T: Clone + 'static> Clone for SignalCompat<T> {
    fn clone(&self) -> Self {
        #[cfg(feature = "leptos-0-6")]
        {
            Self {
                read: self.read.clone(),
                write: self.write.clone(),
                _phantom: PhantomData,
            }
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            Self {
                read: self.read.clone(),
                write: self.write.clone(),
                _phantom: PhantomData,
            }
        }
    }
}

/// Version-agnostic read signal wrapper
pub struct ReadSignalCompat<T: Clone + 'static> {
    #[cfg(feature = "leptos-0-6")]
    inner: leptos_06::ReadSignal<T>,
    
    #[cfg(feature = "leptos-0-8")]
    inner: leptos_08::prelude::ReadSignal<T>,
    
    _phantom: PhantomData<T>,
}

impl<T: Clone + 'static> ReadSignalCompat<T> {
    #[cfg(feature = "leptos-0-6")]
    fn new(inner: leptos_06::ReadSignal<T>) -> Self {
        Self { inner, _phantom: PhantomData }
    }
    
    #[cfg(feature = "leptos-0-8")]
    fn new(inner: leptos_08::prelude::ReadSignal<T>) -> Self {
        Self { inner, _phantom: PhantomData }
    }
    
    /// Get the current value
    pub fn get(&self) -> T {
        self.inner.get()
    }
    
    /// Execute a closure with the current value
    pub fn with<O>(&self, f: impl FnOnce(&T) -> O) -> O {
        #[cfg(feature = "leptos-0-6")]
        {
            self.inner.with(f)
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            self.inner.with(f)
        }
    }
}

impl<T: Clone + 'static> Clone for ReadSignalCompat<T> {
    fn clone(&self) -> Self {
        #[cfg(feature = "leptos-0-6")]
        {
            Self {
                inner: self.inner.clone(),
                _phantom: PhantomData,
            }
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            Self {
                inner: self.inner.clone(),
                _phantom: PhantomData,
            }
        }
    }
}

/// Version-agnostic write signal wrapper
pub struct WriteSignalCompat<T: Clone + 'static> {
    #[cfg(feature = "leptos-0-6")]
    inner: leptos_06::WriteSignal<T>,
    
    #[cfg(feature = "leptos-0-8")]
    inner: leptos_08::prelude::WriteSignal<T>,
    
    _phantom: PhantomData<T>,
}

impl<T: Clone + 'static> WriteSignalCompat<T> {
    #[cfg(feature = "leptos-0-6")]
    fn new(inner: leptos_06::WriteSignal<T>) -> Self {
        Self { inner, _phantom: PhantomData }
    }
    
    #[cfg(feature = "leptos-0-8")]
    fn new(inner: leptos_08::prelude::WriteSignal<T>) -> Self {
        Self { inner, _phantom: PhantomData }
    }
    
    /// Set the signal value
    pub fn set(&self, value: T) {
        self.inner.set(value)
    }
    
    /// Update the signal value using a closure
    pub fn update<F>(&self, f: F) 
    where F: FnOnce(&mut T) {
        self.inner.update(f)
    }
}

impl<T: Clone + 'static> Clone for WriteSignalCompat<T> {
    fn clone(&self) -> Self {
        #[cfg(feature = "leptos-0-6")]
        {
            Self {
                inner: self.inner.clone(),
                _phantom: PhantomData,
            }
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            Self {
                inner: self.inner.clone(),
                _phantom: PhantomData,
            }
        }
    }
}

/// Version-agnostic memo wrapper
pub struct MemoCompat<T: Clone + 'static> {
    #[cfg(feature = "leptos-0-6")]
    inner: leptos_06::Memo<T>,
    
    #[cfg(feature = "leptos-0-8")]
    inner: leptos_08::prelude::Memo<T>,
    
    _phantom: PhantomData<T>,
}

impl<T: Clone + 'static> MemoCompat<T> {
    /// Create a new memo
    pub fn new<F>(f: F) -> Self 
    where F: Fn() -> T + 'static {
        #[cfg(feature = "leptos-0-6")]
        {
            let inner = leptos_06::create_memo(move |_| f());
            Self { inner, _phantom: PhantomData }
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            let inner = leptos_08::prelude::Memo::new(move |_| f());
            Self { inner, _phantom: PhantomData }
        }
    }
    
    /// Get the current memo value
    pub fn get(&self) -> T {
        self.inner.get()
    }
    
    /// Execute a closure with the current value
    pub fn with<O>(&self, f: impl FnOnce(&T) -> O) -> O {
        #[cfg(feature = "leptos-0-6")]
        {
            self.inner.with(f)
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            self.inner.with(f)
        }
    }
}

impl<T: Clone + 'static> Clone for MemoCompat<T> {
    fn clone(&self) -> Self {
        #[cfg(feature = "leptos-0-6")]
        {
            Self {
                inner: self.inner.clone(),
                _phantom: PhantomData,
            }
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            Self {
                inner: self.inner.clone(),
                _phantom: PhantomData,
            }
        }
    }
}

/// Create a signal with version-agnostic API
pub fn signal<T: Clone + 'static>(initial: T) -> SignalCompat<T> {
    SignalCompat::new(initial)
}

/// Create a memo with version-agnostic API
pub fn memo<T: Clone + 'static, F: Fn() -> T + 'static>(f: F) -> MemoCompat<T> {
    MemoCompat::new(f)
}

/// Create a derived signal (memo) with version-agnostic API
pub fn derived<T: Clone + 'static, F: Fn() -> T + 'static>(f: F) -> MemoCompat<T> {
    MemoCompat::new(f)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_signal_creation() {
        let signal = SignalCompat::new(42);
        assert_eq!(signal.get(), 42);
    }
    
    #[test]
    fn test_signal_update() {
        let signal = SignalCompat::new(0);
        signal.set(100);
        assert_eq!(signal.get(), 100);
        
        signal.update(|n| *n += 1);
        assert_eq!(signal.get(), 101);
    }
    
    #[test]
    fn test_memo_creation() {
        let signal = SignalCompat::new(5);
        let memo = MemoCompat::new(move || signal.get() * 2);
        assert_eq!(memo.get(), 10);
    }
    
    #[test]
    fn test_signal_clone() {
        let signal1 = SignalCompat::new("hello");
        let signal2 = signal1.clone();
        
        signal1.set("world");
        assert_eq!(signal1.get(), "world");
        assert_eq!(signal2.get(), "hello"); // Clone should be independent
    }
}
