/*
* Minimal member access call.
* sugar call
* `
* let some = a.b.c;
* foo(d.f.g);
* `
* will be:
* `
* let _x__SDfd1 = a.b;
* let some = _x__SDfd1.c;
* let _x__SDfd2 = d.f
* foo(_x__SDfd2.g)
* `
* in this level, calling method by member access doesnt need, because method calling is
* sugar of regular function calling with self object puting in argument.
*/

use crate::ast::{expr::call_expr::IdentCall, Ident};

#[derive(Debug, Clone)]
pub struct MemberCall {
    object: Box<IdentCall>,
    prop: Box<Ident>,
}

impl MemberCall {
    pub fn new(object: IdentCall, prop: Ident) -> Self {
        Self {
            object: Box::new(object),
            prop: Box::new(prop),
        }
    }
}
