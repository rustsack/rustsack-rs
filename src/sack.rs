use treenode::{TreeNodeValue, TreeNodeInsertable, TreeNodeParent, Instantiatable, TreeNode};
use std::collections::LinkedList;
use std::hash::Hash;
use std::rc::Rc;
use std::fmt::Debug;
use std::borrow::Borrow;
///The properties of a Sack
///A Sack is a refinement of a type-driven tree 
///A Sack is both persistent and immutable (while all immutable trees are persistent, both properties are worth noting) tree
///Since all sacks are immutable sacks are not leaky when open
///An OpenSack is one that can have edges connecting 
///A Terminus is a non-sack immutable object stored inside a sack. Same as a leaf of a tree.

///This is the structure that allows us to build an insanely tightly coupled compile-time tree structure.
///P is the Parent, which must also be a Sack
///S is the Self which can be any object
///C is the ordered set of children of this node
///The three types should always appear in P->S->C order for code clarity not that the P of my C is myself, 
///so this can get rather confusing
///Since TreeNodes are trees themselves (if they have no parent), they all must conform to the fundamental 
///requirements of a tree.
///Even if not implemented as strictly persistent in first implementation, that is the goal, 
///and as such, all operations will consume "Self" and emit a new one.
///A sack is the universe/root of all trees and it is an implementation detail whether a particular sack 
///can transform from being a parent-less universe
///All of these invariants should be enforceable by the type system, and to demonstrate that,
///all invariants should have compiletest-rs tests associated with them.
///All items that are placed in Sacks musts be immutable.
///A wormhole is a non-owning uni-directional link between two sacks
///A pair of wormholes is also the only way to manage mutable state. (IO Monad)
///A shortcut between two nodes in a sack (shorter than iterating the total ordering between A and B) is also a wormhole.
///conceptually it's the act of linking two different stacks. it just happens that in this case, they are just two different points on a stack
///In a weaker type system that can't enforce deep immutability, nesting *any* 
///mutable data structures inside a sack is a bug. Compiler modification might be necessary.
///wormoles, unlike all internal apis, are not versioned, and hence truly do change behavior 
///when something causes the previous version of the sack to become a new version
///while internal messages are *excruciatingly* strongly typed, message sent through wormholes are only duck typed
//pub struct Sack<P, S, C>
//    where P: SackParent<S>,
//          S: Sized + PartialEq + Eq + Hash + SackInsertable
//{
//    ///child->parent relationships are necessarily weak
//    p: Rc<P>,
//    ///A generic treenode can have any value, but the value type needs to be able to be put inside of a parent's hash
//    s: Box<S>,
//    ///This should ideally be a non-distributed immutable/persistent tree. 
//    ///Will approximate O(log N) insert and retrieve with O(N) ones for now
//    c: LinkedList<C>,
//}

pub type RustSack<P, S, C> = TreeNode<SackParent<P>, S, C>;
trait Generator<C> {
	fn add_node(self, C) -> Self;
}

pub type Sack<P, S, C> = TreeNode<P, S, C>;
// While not interesting currently, this where be where the
// interesting aspects of a sack get defined, such as wormholes

pub type SackValue<V> = TreeNodeValue<V>;

///All TreeNodes that are Addressable are guaranteed to have Sack-unique fqdns
pub trait Addressable{
	fn fqdn(&self) -> String;
	fn node_type(&self) -> String;
}

impl Addressable for () {
	fn fqdn(&self) -> String {"".to_owned()}
	fn node_type(&self) -> String {"".to_owned()}
}

impl<P,S,C> Addressable for Sack<P,S,C> where P : TreeNodeParent<S> + Addressable, S: PartialEq+ Eq+ Hash {
	fn fqdn(&self) -> String {
		let p:&P = self.p.borrow();
		p.node_type()
		//	Err(_) => panic!("can't reach my parent. this is a bug in a single process system, and split brain frozenness in a distributed one")
		}
	
	fn node_type(&self) -> String {
		"Sack".to_owned()
	}
}

//impl Addressable for (){
//	fn fqdn(&self) -> String {"".to_owned()}
//	fn node_type(&self) -> String {"".to_owned()}
//}



pub trait SackInsertable: TreeNodeInsertable + PartialEq + Eq + Hash {}
impl<S> SackInsertable for S where S: PartialEq + Eq + Hash + TreeNodeInsertable {}


///A finite set of all types of things that can be put in a treenode
pub enum SackItem<P, S, C>
    where P: SackInsertable + SackParent<SackValue<S>> + TreeNodeParent<TreeNodeValue<S>>,
          S: SackCarryable,
          SackValue<S>: SackInsertable
{
    Sack(Sack<P, SackValue<S>, C>),
    POSO(Sack<P, SackValue<S>, ()>), // Plain Old Sack Object. Anything that can be put in a sack.
}

pub trait SackCarryable: PartialEq + Eq + Hash {}

impl Instantiatable for Sack<(),(), ()>
 {
    fn new() -> Sack<(),(),()> {
        Sack {
            p: Rc::new(()),
            s: Box::new(()),
            c: LinkedList::new(),
        }
    }
}

///For now, only strings 
impl SackCarryable for String {}

type NodeName = SackValue<String>;

pub trait SackParent<C: SackInsertable>: SackInsertable {
    // fn insert_child(self, C) -> Self;
}

impl<S> SackParent<S> for () where S: PartialEq + Eq + Hash {}
impl<S> SackParent<S> for String where S: PartialEq + Eq + Hash {}

#[test]
fn can_instantiate() {
    let _ = Sack::new();
}

#[test]
fn default_fqdn_is_empty() {
	assert_eq!(Sack::new().fqdn(),"");
}