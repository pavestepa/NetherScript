use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
struct _Ag1 {
  pub num: f64
}

pub fn case1() {
  let a = Rc::new(RefCell::new( _Ag1{ num: 1.0 }));
  let b = a.clone();
  let c = a.clone();

  a.borrow_mut().num = 2.0;
  b.borrow_mut().num = 4.0;
  c.borrow_mut().num = 5.0;

  println!("{:#?}", &a);
  println!("{:#?}", &b);
  println!("{:#?}", &c);

  *c.borrow_mut() = _Ag1{ num: 10.0 };

  println!("{:#?}", c);
}