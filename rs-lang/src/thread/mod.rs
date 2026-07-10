//! # 多线程核心：Sync + Send
//! Send 标记 trait 表明实现了 Send 的类型值的所有权可以在线程间转移.
//! Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中同时拥有其值的引用.
//!
//! 对于任意类型 T，如果 &T（T 的不可变引用）实现了 Send 的话 T 就实现了 Sync，反过来也成立：
//! **T 实现了 Sync** <===> **&T  实现了 Send**
//!
//! 只有当类型 T 的引用 (&T) 可以安全地发送 (Send) 到另一个线程时，T 才是线程安全的 (Sync)。
//! 描述是否线程安全时需要区分数据类型本身以及对数据的操作。绝大多数数据类型是线程安全的，如i32、String，但对数据
//! 进行操作时不是线程安全，如 ``count += 1;``;而有的数据类型本身就不是线程安全的，如 `Rc`
pub mod message_passing;
pub mod multi_threads;
pub mod pool;
pub mod shared_state;
