use crate::{IndexMap, IndexSet};
use gc::{custom_trace, Finalize, Trace};
use std::hash::{BuildHasher, Hash};

impl<K: Eq + Hash + Trace, V: Trace, S: BuildHasher> Finalize for IndexMap<K, V, S> {}
unsafe impl<K: Eq + Hash + Trace, V: Trace, S: BuildHasher> Trace for IndexMap<K, V, S> {
    custom_trace!(this, {
        for (k, v) in this.iter() {
            mark(k);
            mark(v);
        }
    });
}
