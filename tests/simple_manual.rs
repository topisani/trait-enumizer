trait MyIface {
    fn foo(&self);
    fn bar(&self, x: i32);
    fn baz(&self, y: String, z: Vec<u8>);
}

struct Implementor {}

impl MyIface for Implementor {
    fn foo(&self) {
        dbg!("foo");
    }

    fn bar(&self, x: i32) {
        dbg!("bar", x);
    }

    fn baz(&self, y: String, z: Vec<u8>) {
        dbg!("baz", y, z);
    }
}

enum MyIfaceEnum {
    Foo,
    Bar { x: i32 },
    Baz { y: String, z: Vec<u8> },
}

impl MyIfaceEnum {
    fn call<I: MyIface>(self, o: &I) {
        match self {
            MyIfaceEnum::Foo => o.foo(),
            MyIfaceEnum::Bar { x } => o.bar(x),
            MyIfaceEnum::Baz { y, z } => o.baz(y, z),
        }
    }
}
trait MyIfaceResultified<E> {
    fn try_foo(&self) -> Result<(), E>;
    fn try_bar(&self, x: i32) -> Result<(), E>;
    fn try_baz(&self, y: String, z: Vec<u8>) -> Result<(), E>;
}

impl<R:MyIfaceResultified<std::convert::Infallible>> MyIface for R {
    fn foo(&self) {
        R::try_foo(self).unwrap()
    }

    fn bar(&self, x: i32) {
        R::try_bar(self, x).unwrap()
    }

    fn baz(&self, y: String, z: Vec<u8>) {
        R::try_baz(self,y,z).unwrap()
    }
}

struct MyIfaceProxy<E, F: Fn(MyIfaceEnum)-> Result<(), E> > (F);
impl<E, F: Fn(MyIfaceEnum) -> Result<(), E>> MyIfaceResultified<E> for MyIfaceProxy<E, F> {
    fn try_foo(&self) -> Result<(), E> {
        self.0(MyIfaceEnum::Foo)
    }

    fn try_bar(&self, x: i32) -> Result<(), E> {
        self.0(MyIfaceEnum::Bar { x })
    }

    fn try_baz(&self, y: String, z: Vec<u8>) -> Result<(), E> {
        self.0(MyIfaceEnum::Baz { y, z })
    }
}

#[test]
fn simple() {
    let o = Implementor {};
    let p = MyIfaceProxy(|c| Ok(c.call(&o)));
    p.foo();
    p.bar(4);
}
