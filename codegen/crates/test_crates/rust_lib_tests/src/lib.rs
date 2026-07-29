pub trait SimpleTrait {}

pub trait WithAssocItem {
    type A: SimpleTrait;
}

impl SimpleTrait for usize {}
impl WithAssocItem for usize {
    type A = usize;
}

pub struct TargetType;

#[allow(unused_variables)]
impl TargetType {
    pub fn simple_fn(arg: usize) {

    }

    pub fn simple_fn_negative(arg: String) {

    }


    pub fn with_assoc_fn<A: WithAssocItem>(arg: A) {

    }
}
