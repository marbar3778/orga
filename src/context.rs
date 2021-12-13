#[cfg(test)]
use mutagen::mutate;

use crate::state::State;
use std::any::TypeId;
use std::collections::HashMap;
use std::lazy::SyncLazy;
use std::mem::{transmute, ManuallyDrop};
use std::sync::Mutex;

type ContextMap = ManuallyDrop<HashMap<TypeId, Box<()>>>;
static CONTEXT_MAP: SyncLazy<Mutex<ContextMap>> =
    SyncLazy::new(|| Mutex::new(ManuallyDrop::new(HashMap::new())));

pub struct Context<I> {
    _inner: I,
}

impl Context<()> {
    #[cfg_attr(test, mutate)]
    pub fn add<T: 'static>(ctx: T) {
        let mut context_store = CONTEXT_MAP.lock().unwrap();
        let id = TypeId::of::<T>();
        let boxed_ctx = Box::new(ctx);
        let raw = unsafe { transmute::<_, Box<()>>(boxed_ctx) };
        let replaced = context_store.insert(id, raw);
        if let Some(replaced) = replaced {
            unsafe { transmute::<_, Box<T>>(replaced) };
        }
    }

    #[cfg_attr(test, mutate)]
    pub fn resolve<'a, T: 'static>() -> Option<&'a mut T> {
        let mut context_store = CONTEXT_MAP.lock().unwrap();
        let id = TypeId::of::<T>();
        let boxed_ctx = context_store.get_mut(&id);
        match boxed_ctx {
            Some(ctx) => unsafe { Some(transmute::<_, &'a mut Box<T>>(ctx)) },
            None => None,
        }
    }

    #[cfg_attr(test, mutate)]
    pub fn remove<T: 'static>() {
        let mut context_store = CONTEXT_MAP.lock().unwrap();
        if let Some(replaced) = context_store.remove(&TypeId::of::<T>()) {
            unsafe { transmute::<_, Box<T>>(replaced) };
        }
    }
}

pub trait GetContext {
    fn context<T: 'static>(&mut self) -> Option<&mut T>;
}

impl<S: State> GetContext for S {
    fn context<T: 'static>(&mut self) -> Option<&mut T> {
        Context::resolve::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct ContextA {
        foo: u32,
    }

    struct ContextB {
        bar: Vec<u8>,
    }

    struct ContextC {
        _baz: u32,
    }
    #[test]
    fn context_add_and_resolve() {
        let a = ContextA { foo: 0 };
        let b = ContextA { foo: 1 };
        let c = ContextA { foo: 2 };
        let d = ContextB { bar: vec![1, 2, 3] };
        Context::add(a);
        Context::add(b);
        Context::add(c);
        Context::add(d);
        let resolved_a = Context::resolve::<ContextA>().unwrap();
        let resolved_b = Context::resolve::<ContextB>().unwrap();
        let resolved_c = Context::resolve::<ContextC>();
        assert_eq!(resolved_a.foo, 2);
        assert_eq!(resolved_b.bar, vec![1, 2, 3]);
        assert!(resolved_c.is_none());
    }
}