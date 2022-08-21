(function() {var implementors = {
"aero_kernel":[["impl&lt;T:&nbsp;?<a class=\"trait\" href=\"aero_kernel/prelude/rust_2021/_core/marker/trait.Sized.html\" title=\"trait aero_kernel::prelude::rust_2021::_core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"aero_kernel/prelude/rust_2021/_core/ops/trait.DerefMut.html\" title=\"trait aero_kernel::prelude::rust_2021::_core::ops::DerefMut\">DerefMut</a> for <a class=\"struct\" href=\"aero_kernel/drivers/block/nvme/dma/struct.Dma.html\" title=\"struct aero_kernel::drivers::block::nvme::dma::Dma\">Dma</a>&lt;T&gt;"],["impl&lt;'a, T:&nbsp;?<a class=\"trait\" href=\"aero_kernel/prelude/rust_2021/_core/marker/trait.Sized.html\" title=\"trait aero_kernel::prelude::rust_2021::_core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"aero_kernel/prelude/rust_2021/_core/ops/trait.DerefMut.html\" title=\"trait aero_kernel::prelude::rust_2021::_core::ops::DerefMut\">DerefMut</a> for <a class=\"struct\" href=\"aero_kernel/utils/sync/struct.MutexGuard.html\" title=\"struct aero_kernel::utils::sync::MutexGuard\">MutexGuard</a>&lt;'a, T&gt;"]],
"arrayvec":[["impl&lt;T, const CAP:&nbsp;usize&gt; DerefMut for <a class=\"struct\" href=\"arrayvec/struct.ArrayVec.html\" title=\"struct arrayvec::ArrayVec\">ArrayVec</a>&lt;T, CAP&gt;"],["impl&lt;const CAP:&nbsp;usize&gt; DerefMut for <a class=\"struct\" href=\"arrayvec/struct.ArrayString.html\" title=\"struct arrayvec::ArrayString\">ArrayString</a>&lt;CAP&gt;"]],
"lock_api":[["impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a> + 'a, T:&nbsp;?Sized + 'a&gt; DerefMut for <a class=\"struct\" href=\"lock_api/struct.MutexGuard.html\" title=\"struct lock_api::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;"],["impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a> + 'a, T:&nbsp;?Sized + 'a&gt; DerefMut for <a class=\"struct\" href=\"lock_api/struct.MappedMutexGuard.html\" title=\"struct lock_api::MappedMutexGuard\">MappedMutexGuard</a>&lt;'a, R, T&gt;"],["impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a> + 'a, T:&nbsp;?Sized + 'a&gt; DerefMut for <a class=\"struct\" href=\"lock_api/struct.RwLockWriteGuard.html\" title=\"struct lock_api::RwLockWriteGuard\">RwLockWriteGuard</a>&lt;'a, R, T&gt;"],["impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a> + 'a, T:&nbsp;?Sized + 'a&gt; DerefMut for <a class=\"struct\" href=\"lock_api/struct.MappedRwLockWriteGuard.html\" title=\"struct lock_api::MappedRwLockWriteGuard\">MappedRwLockWriteGuard</a>&lt;'a, R, T&gt;"]],
"once_cell":[["impl&lt;T, F:&nbsp;FnOnce() -&gt; T&gt; DerefMut for <a class=\"struct\" href=\"once_cell/unsync/struct.Lazy.html\" title=\"struct once_cell::unsync::Lazy\">Lazy</a>&lt;T, F&gt;"]],
"scopeguard":[["impl&lt;T, F, S&gt; DerefMut for <a class=\"struct\" href=\"scopeguard/struct.ScopeGuard.html\" title=\"struct scopeguard::ScopeGuard\">ScopeGuard</a>&lt;T, F, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: FnOnce(T),<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"scopeguard/trait.Strategy.html\" title=\"trait scopeguard::Strategy\">Strategy</a>,&nbsp;</span>"]],
"spin":[["impl&lt;'a, T:&nbsp;?Sized&gt; DerefMut for <a class=\"struct\" href=\"spin/mutex/spin/struct.SpinMutexGuard.html\" title=\"struct spin::mutex::spin::SpinMutexGuard\">SpinMutexGuard</a>&lt;'a, T&gt;"],["impl&lt;'a, T:&nbsp;?Sized&gt; DerefMut for <a class=\"struct\" href=\"spin/mutex/struct.MutexGuard.html\" title=\"struct spin::mutex::MutexGuard\">MutexGuard</a>&lt;'a, T&gt;"],["impl&lt;'rwlock, T:&nbsp;?Sized, R&gt; DerefMut for <a class=\"struct\" href=\"spin/rwlock/struct.RwLockWriteGuard.html\" title=\"struct spin::rwlock::RwLockWriteGuard\">RwLockWriteGuard</a>&lt;'rwlock, T, R&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()