//! `push-pull` defines the traits `PullFrom` and `PushInto` that allow
//! types to exchange data. This is analogous to the built-in traits `From`
//! and `Into` but with the difference that no new instances are created
//! but existing instances are modified.
//!
//! ## Example
//! ```rust
//! use push_pull::{PullFrom, PushInto};
//!
//! #[derive(Debug)]
//! struct Vec2 {
//!     x: i32,
//!     y: i32
//! }
//!
//! #[derive(Debug, PartialEq)]
//! struct Vec3 {
//!     x: i32,
//!     y: i32,
//!     z: i32
//! }
//!
//! impl PullFrom<Vec2> for Vec3 {
//!     fn pull_from(&mut self, vec_2: &Vec2) {
//!         self.x = vec_2.x;
//!         self.y = vec_2.y;
//!     }
//! }
//!
//! fn main() {
//!     let vec_2 = Vec2 { x: -23, y: 48 };
//!     let mut vec_3 = Vec3 { x: 43, y: -64, z: 104};
//!     
//!     vec_3.pull_from(&vec_2);
//!     assert_eq!(vec_3, Vec3 { x: -23, y: 48, z: 104 });
//!
//!     let vec_2 = Vec2 { x: 0, y: 0 };
//!
//!     // `PushInto` has a blanket implementation for types that
//!     // implement `PullFrom`.
//!     vec_2.push_into(&mut vec_3);
//!     assert_eq!(vec_3, Vec3 { x: 0, y: 0, z: 104 });
//!
//! }
//! ```

/// Pull data from a given source.
pub trait PullFrom<S: ?Sized> {
    /// # Example
    /// ```
    /// let vec_2 = Vec2 { x: -23, y: 48 };
    /// let mut vec_3 = Vec3 { x: 43, y: -64, z: 104};
    ///
    /// vec_3.pull_from(&vec_2);
    ///
    /// assert_eq!(vec_3, Vec3 { x: -23, y: 48, z: 104 });
    /// ```
    fn pull_from(&mut self, source: &S);
}

/// Push data into a given target.
pub trait PushInto<T: ?Sized> {
    /// # Example
    /// ```
    /// let vec_2 = Vec2 { x: -23, y: 48 };
    /// let mut vec_3 = Vec3 { x: 43, y: -64, z: 104};
    ///
    /// vec_2.push_into(&mut vec3);
    ///
    /// assert_eq!(vec_3, Vec3 { x: -23, y: 48, z: 104 });
    /// ```
    fn push_into(&self, target: &mut T);
}

impl<T: PullFrom<S>, S> PushInto<T> for S {
    fn push_into(&self, target: &mut T) {
        target.pull_from(self);
    }
}
