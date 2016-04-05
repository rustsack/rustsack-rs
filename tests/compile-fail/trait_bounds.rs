trait Foo {
    fn dummy(&self) { }
}

fn a(_x: Box<Foo+Send>) {
}

fn c(x: Box<Foo+Sync+Send>) {
    a(x);
}

fn d(x: Box<Foo>) {
    a(x); //~  ERROR mismatched types
          //~| expected `Box<Foo + Send + 'static>`
          //~| found `Box<Foo + 'static>`
          //~| expected bounds `Send`
          //~| found no bounds
}
