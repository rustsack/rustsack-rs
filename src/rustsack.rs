use treenode::TreeNode;//, TreeNodeInsertable, NilParent, TreeNodeValue, TreeNodeParent};
use sack::{SackValue, SackInsertable};
use std::collections::LinkedList;
use std::any::Any;

// #[derive(PartialEq,Eq, Hash)]
// pub enum RustSackValue {
//    Name(NodeName),
// }

pub type RustSackValue<V> = SackValue<V>;
pub type RustSack<P, C> = TreeNode<RustSackParent<P>, SackValue<RustSackValue<P>>, C>;
trait Generator<C> {
	fn add_node(self, C) -> Self;
}

// /impl RustSack<P> {}
// pub type RustSackRoot = TreeNode<NilParent, SackValue<RustSackValue>, RustSackChild>;
//
// impl TreeNodeInsertable for TreeNodeValue<RustSackValue> {}
//
pub trait RustSackInsertable: SackInsertable {}

// /impl TreeNodeInsertable for RustSackInsertable {}
// impl RustSackInsertable for RustSackChild{}
// impl SackInsertable for RustSackChild {}
//
pub trait RustSackParent<C: RustSackInsertable>: RustSackInsertable {
    fn insert_child(self, C) -> Self;
}
// impl TreeNodeInsertable for RustSackParent<Sized> {}
// impl TreeNodeInsertable for RustSackValue {}
//
// impl TreeNodeParent<RustSackValue> for RustSackParent<Any> {
//    fn insert_child(self, child: RustSackValue) -> RustSackParent<Any> {}
// }
//
//
// #[derive(PartialEq,Eq, Hash)]
// pub enum RustSackChild {
//    RustSack(RustSack<Sized>),
//    ///PORO == Plain Old RustSack Object, and is any non-RustSack that can be put in a RustSack (and hence any that can be observed/enumerated by one)
//    PORO(()),
// }

// impl TreeNodeInsertable for RustSackChild{}
//
// impl RustSack<Sized> {}
//
#[test]
fn can_instantiate() {
    // let _: RustSack = TreeNode{p:Weak(()),s:Box("Hello".to_owned()),c:LinkedList{}};
}
