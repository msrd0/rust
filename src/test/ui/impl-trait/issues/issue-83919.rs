#![feature(type_alias_impl_trait)]

// edition:2021

use std::future::Future;

trait Foo {
    type T;
    type Fut2: Future<Output=Self::T>; // ICE got triggered with traits other than Future here
    type Fut: Future<Output=Self::Fut2>;
    fn get_fut(&self) -> Self::Fut;
}

struct Implementor;

impl Foo for Implementor {
    type T = u64;
    type Fut2 = impl Future<Output=u64>;
    type Fut = impl Future<Output=Self::Fut2>;

    fn get_fut(&self) -> Self::Fut {
        async move {
            42 //~^ ERROR `{integer}` is not a future
            // 42 does not impl Future and rustc does actually point out the error,
            // but rustc used to panic.
            // Putting a valid Future here always worked fine.
        }
    }
}

fn main() {}
