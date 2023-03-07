(function() {var implementors = {
"aero_kernel":[["impl <a class=\"trait\" href=\"aero_kernel/prelude/rust_2021/_core/ops/trait.Drop.html\" title=\"trait aero_kernel::prelude::rust_2021::_core::ops::Drop\">Drop</a> for <a class=\"struct\" href=\"aero_kernel/fs/block/struct.CachedPage.html\" title=\"struct aero_kernel::fs::block::CachedPage\">CachedPage</a>"],["impl <a class=\"trait\" href=\"aero_kernel/prelude/rust_2021/_core/ops/trait.Drop.html\" title=\"trait aero_kernel::prelude::rust_2021::_core::ops::Drop\">Drop</a> for <a class=\"struct\" href=\"aero_kernel/utils/sync/struct.IrqGuard.html\" title=\"struct aero_kernel::utils::sync::IrqGuard\">IrqGuard</a>"],["impl&lt;T: <a class=\"trait\" href=\"aero_kernel/fs/cache/trait.CacheDropper.html\" title=\"trait aero_kernel::fs::cache::CacheDropper\">CacheDropper</a>&gt; <a class=\"trait\" href=\"aero_kernel/prelude/rust_2021/_core/ops/trait.Drop.html\" title=\"trait aero_kernel::prelude::rust_2021::_core::ops::Drop\">Drop</a> for <a class=\"struct\" href=\"aero_kernel/fs/cache/struct.CacheArc.html\" title=\"struct aero_kernel::fs::cache::CacheArc\">CacheArc</a>&lt;T&gt;"],["impl <a class=\"trait\" href=\"aero_kernel/prelude/rust_2021/_core/ops/trait.Drop.html\" title=\"trait aero_kernel::prelude::rust_2021::_core::ops::Drop\">Drop</a> for <a class=\"struct\" href=\"aero_kernel/fs/inode/struct.PollTable.html\" title=\"struct aero_kernel::fs::inode::PollTable\">PollTable</a>"],["impl <a class=\"trait\" href=\"aero_kernel/prelude/rust_2021/_core/ops/trait.Drop.html\" title=\"trait aero_kernel::prelude::rust_2021::_core::ops::Drop\">Drop</a> for <a class=\"struct\" href=\"aero_kernel/fs/ext2/group_desc/struct.Bitmap.html\" title=\"struct aero_kernel::fs::ext2::group_desc::Bitmap\">Bitmap</a>"],["impl <a class=\"trait\" href=\"aero_kernel/prelude/rust_2021/_core/ops/trait.Drop.html\" title=\"trait aero_kernel::prelude::rust_2021::_core::ops::Drop\">Drop</a> for <a class=\"struct\" href=\"aero_kernel/net/struct.RecvPacket.html\" title=\"struct aero_kernel::net::RecvPacket\">RecvPacket</a>"],["impl&lt;'a, T: ?<a class=\"trait\" href=\"aero_kernel/prelude/rust_2021/_core/marker/trait.Sized.html\" title=\"trait aero_kernel::prelude::rust_2021::_core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"aero_kernel/prelude/rust_2021/_core/ops/trait.Drop.html\" title=\"trait aero_kernel::prelude::rust_2021::_core::ops::Drop\">Drop</a> for <a class=\"struct\" href=\"aero_kernel/utils/sync/struct.BMutexGuard.html\" title=\"struct aero_kernel::utils::sync::BMutexGuard\">BMutexGuard</a>&lt;'a, T&gt;"],["impl&lt;'a, T: ?<a class=\"trait\" href=\"aero_kernel/prelude/rust_2021/_core/marker/trait.Sized.html\" title=\"trait aero_kernel::prelude::rust_2021::_core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"aero_kernel/prelude/rust_2021/_core/ops/trait.Drop.html\" title=\"trait aero_kernel::prelude::rust_2021::_core::ops::Drop\">Drop</a> for <a class=\"struct\" href=\"aero_kernel/utils/sync/struct.MutexGuard.html\" title=\"struct aero_kernel::utils::sync::MutexGuard\">MutexGuard</a>&lt;'a, T&gt;"]],
"arrayvec":[["impl&lt;'a, T: 'a, const CAP: usize&gt; Drop for <a class=\"struct\" href=\"arrayvec/struct.Drain.html\" title=\"struct arrayvec::Drain\">Drain</a>&lt;'a, T, CAP&gt;"],["impl&lt;T, const CAP: usize&gt; Drop for <a class=\"struct\" href=\"arrayvec/struct.IntoIter.html\" title=\"struct arrayvec::IntoIter\">IntoIter</a>&lt;T, CAP&gt;"],["impl&lt;T, const CAP: usize&gt; Drop for <a class=\"struct\" href=\"arrayvec/struct.ArrayVec.html\" title=\"struct arrayvec::ArrayVec\">ArrayVec</a>&lt;T, CAP&gt;"]],
"hashbrown":[["impl&lt;'a, K, F, A: Allocator + Clone&gt; Drop for <a class=\"struct\" href=\"hashbrown/hash_set/struct.DrainFilter.html\" title=\"struct hashbrown::hash_set::DrainFilter\">DrainFilter</a>&lt;'a, K, F, A&gt;<span class=\"where fmt-newline\">where\n    F: FnMut(&amp;K) -&gt; bool,</span>"],["impl&lt;'a, K, V, F, A&gt; Drop for <a class=\"struct\" href=\"hashbrown/hash_map/struct.DrainFilter.html\" title=\"struct hashbrown::hash_map::DrainFilter\">DrainFilter</a>&lt;'a, K, V, F, A&gt;<span class=\"where fmt-newline\">where\n    F: FnMut(&amp;K, &amp;mut V) -&gt; bool,\n    A: Allocator + Clone,</span>"]],
"intrusive_collections":[["impl&lt;A: <a class=\"trait\" href=\"intrusive_collections/trait.Adapter.html\" title=\"trait intrusive_collections::Adapter\">Adapter</a>&gt; Drop for <a class=\"struct\" href=\"intrusive_collections/linked_list/struct.LinkedList.html\" title=\"struct intrusive_collections::linked_list::LinkedList\">LinkedList</a>&lt;A&gt;<span class=\"where fmt-newline\">where\n    A::<a class=\"associatedtype\" href=\"intrusive_collections/trait.Adapter.html#associatedtype.LinkOps\" title=\"type intrusive_collections::Adapter::LinkOps\">LinkOps</a>: <a class=\"trait\" href=\"intrusive_collections/linked_list/trait.LinkedListOps.html\" title=\"trait intrusive_collections::linked_list::LinkedListOps\">LinkedListOps</a>,</span>"],["impl&lt;A: <a class=\"trait\" href=\"intrusive_collections/trait.Adapter.html\" title=\"trait intrusive_collections::Adapter\">Adapter</a>&gt; Drop for <a class=\"struct\" href=\"intrusive_collections/xor_linked_list/struct.XorLinkedList.html\" title=\"struct intrusive_collections::xor_linked_list::XorLinkedList\">XorLinkedList</a>&lt;A&gt;<span class=\"where fmt-newline\">where\n    A::<a class=\"associatedtype\" href=\"intrusive_collections/trait.Adapter.html#associatedtype.LinkOps\" title=\"type intrusive_collections::Adapter::LinkOps\">LinkOps</a>: <a class=\"trait\" href=\"intrusive_collections/xor_linked_list/trait.XorLinkedListOps.html\" title=\"trait intrusive_collections::xor_linked_list::XorLinkedListOps\">XorLinkedListOps</a>,</span>"],["impl&lt;A: <a class=\"trait\" href=\"intrusive_collections/trait.Adapter.html\" title=\"trait intrusive_collections::Adapter\">Adapter</a>&gt; Drop for <a class=\"struct\" href=\"intrusive_collections/singly_linked_list/struct.SinglyLinkedList.html\" title=\"struct intrusive_collections::singly_linked_list::SinglyLinkedList\">SinglyLinkedList</a>&lt;A&gt;<span class=\"where fmt-newline\">where\n    A::<a class=\"associatedtype\" href=\"intrusive_collections/trait.Adapter.html#associatedtype.LinkOps\" title=\"type intrusive_collections::Adapter::LinkOps\">LinkOps</a>: <a class=\"trait\" href=\"intrusive_collections/singly_linked_list/trait.SinglyLinkedListOps.html\" title=\"trait intrusive_collections::singly_linked_list::SinglyLinkedListOps\">SinglyLinkedListOps</a>,</span>"],["impl&lt;A: <a class=\"trait\" href=\"intrusive_collections/trait.Adapter.html\" title=\"trait intrusive_collections::Adapter\">Adapter</a>&gt; Drop for <a class=\"struct\" href=\"intrusive_collections/rbtree/struct.RBTree.html\" title=\"struct intrusive_collections::rbtree::RBTree\">RBTree</a>&lt;A&gt;<span class=\"where fmt-newline\">where\n    A::<a class=\"associatedtype\" href=\"intrusive_collections/trait.Adapter.html#associatedtype.LinkOps\" title=\"type intrusive_collections::Adapter::LinkOps\">LinkOps</a>: <a class=\"trait\" href=\"intrusive_collections/rbtree/trait.RBTreeOps.html\" title=\"trait intrusive_collections::rbtree::RBTreeOps\">RBTreeOps</a>,</span>"]],
"lru":[["impl&lt;K, V, S&gt; Drop for <a class=\"struct\" href=\"lru/struct.LruCache.html\" title=\"struct lru::LruCache\">LruCache</a>&lt;K, V, S&gt;"]],
"once_cell":[["impl&lt;T&gt; Drop for <a class=\"struct\" href=\"once_cell/race/struct.OnceBox.html\" title=\"struct once_cell::race::OnceBox\">OnceBox</a>&lt;T&gt;"]],
"spin":[["impl&lt;'rwlock, T: ?Sized, R&gt; Drop for <a class=\"struct\" href=\"spin/rwlock/struct.RwLockUpgradableGuard.html\" title=\"struct spin::rwlock::RwLockUpgradableGuard\">RwLockUpgradableGuard</a>&lt;'rwlock, T, R&gt;"],["impl&lt;'rwlock, T: ?Sized&gt; Drop for <a class=\"struct\" href=\"spin/rwlock/struct.RwLockReadGuard.html\" title=\"struct spin::rwlock::RwLockReadGuard\">RwLockReadGuard</a>&lt;'rwlock, T&gt;"],["impl&lt;'a, T: ?Sized&gt; Drop for <a class=\"struct\" href=\"spin/mutex/spin/struct.SpinMutexGuard.html\" title=\"struct spin::mutex::spin::SpinMutexGuard\">SpinMutexGuard</a>&lt;'a, T&gt;"],["impl&lt;'rwlock, T: ?Sized, R&gt; Drop for <a class=\"struct\" href=\"spin/rwlock/struct.RwLockWriteGuard.html\" title=\"struct spin::rwlock::RwLockWriteGuard\">RwLockWriteGuard</a>&lt;'rwlock, T, R&gt;"],["impl&lt;T, R&gt; Drop for <a class=\"struct\" href=\"spin/once/struct.Once.html\" title=\"struct spin::once::Once\">Once</a>&lt;T, R&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()