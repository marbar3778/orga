use crate::error::Result;
use crate::store::{BufStore, Read, Shared, Write};

/// A helper which runs state machine logic, discarding the writes to the store
/// for error results and flushing to the underlying store on success.
pub fn step_atomic<S, F, I, O>(_f: F, _store: Option<S>, _input: I) -> Result<O>
where
    S: Read + Write,
    F: Fn(Shared<BufStore<S>>, I) -> Result<O>,
{
    unimplemented!();
    // let owned_store = store.take().unwrap();
    // let flush_store = Shared::new(BufStore::wrap(owned_store.clone()));
    // let res = f(flush_store.clone(), input)?;
    // let mut unwrapped_fs = flush_store.into_inner();
    // unwrapped_fs.flush()?;
    // store.replace(owned_store);
    // Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::{MapStore, Read, Write};
    use crate::Error;

    fn get_u8<R: Read>(key: &[u8], store: R) -> Result<u8> {
        match store.get(key) {
            Ok(None) => Ok(0),
            Ok(Some(vec)) => Ok(vec[0]),
            Err(err) => Err(err),
        }
    }

    fn put_u8<W: Write>(key: &[u8], value: u8, mut store: W) -> Result<()> {
        store.put(key.to_vec(), vec![value])
    }

    fn counter<S: Read + Write>(mut store: S, n: u8) -> Result<u8> {
        // put to n before checking count (to test atomicity)
        put_u8(b"n", n, &mut store)?;

        // get count, compare to n, write if successful
        let count = get_u8(b"count", &store)?;
        if count != n {
            return Err(Error::Test("Invalid count".into()));
        }
        put_u8(b"count", count + 1, &mut store)?;
        Ok(count + 1)
    }

    #[test]
    fn step_counter_error() {
        let mut store = MapStore::new();
        // invalid `n`, should error
        assert!(counter(&mut store, 100).is_err());
        // count should not have been mutated
        assert_eq!(store.get(b"count").unwrap(), None);
        // n should have been mutated
        assert_eq!(store.get(b"n").unwrap(), Some(vec![100]));
    }

    // #[test]
    // fn step_counter_atomic_error() {
    //     let mut store = MapStore::new();
    //     // invalid `n`, should error
    //     assert!(step_atomic(|s, i| counter(s, i), &mut store, 100).is_err());
    //     // count should not have been mutated
    //     assert_eq!(store.get(b"count").unwrap(), None);
    //     // n should not have been mutated
    //     assert_eq!(store.get(b"n").unwrap(), None);
    // }

    // #[test]
    // fn step_counter() {
    //     let mut store = MapStore::new();
    //     assert_eq!(step_atomic(|s, i| counter(s, i), &mut store, 0).unwrap(), 1);
    //     assert!(step_atomic(|s, i| counter(s, i), &mut store, 0).is_err());
    //     assert_eq!(step_atomic(|s, i| counter(s, i), &mut store, 1).unwrap(), 2);
    //     assert!(step_atomic(|s, i| counter(s, i), &mut store, 1).is_err());
    //     assert_eq!(store.get(b"n").unwrap(), Some(vec![1]));
    //     assert_eq!(store.get(b"count").unwrap(), Some(vec![2]));
    // }

    // #[test]
    // fn closure_sm() {
    //     let mut store = MapStore::new();
    //     assert_eq!(
    //         step_atomic(|_, input| Ok(input + 1), &mut store, 100).unwrap(),
    //         101
    //     );
    // }
}
