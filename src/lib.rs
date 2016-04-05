use std::any::Any;
use std::hash::Hash;
use std::collections::LinkedList;
use std::rc::Weak;
use std::hash::Hasher;

fn main() {
   // let mysack = RustSack{c:1,s:2,p:3};
}


///The properties of a Sack
///A Sack is a refinement of a type-driven tree 
///A Sack is both persistent and immutable (while all immutable trees are persistent, both properties are worth noting) tree
///Since all sacks are immutable sacks are not leaky when open
///An OpenSack is one that can have edges connecting 
///A Terminus is a non-sack immutable object stored inside a sack. Same as a leaf of a tree.


///This is the structure that allows us to build an insanely tightly coupled compile-time tree structure.
///P is the Parent, which must also be a treenod
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
#[derive(Clone)]
pub struct TreeNode<P,S,C> where P : TreeNodeInsertable, S : Sized + PartialEq + Eq + Hash {
	p:Weak<P>,
	s:Box<S>,
	c:LinkedList<C>
}

///Anything that can be the child of a treenode has to be Hash-able
pub trait TreeNodeInsertable : Sized + PartialEq + Eq + Hash{} 

///We need to implement our own PartialEq because we have to ignore the weak parent ref
impl<P,S,C> PartialEq for TreeNode<P,S,C> where P: TreeNodeInsertable, C : TreeNodeInsertable, S:PartialEq + Eq + Hash {
	fn eq(&self, other:&TreeNode<P,S,C>) -> bool {
		self.s == other.s && self.c == other.c
	}	
}

///We need to implement our own Eq because we have to ignore the weak parent ref
impl<P,S,C> Eq for TreeNode<P,S,C> where P: TreeNodeInsertable, C : TreeNodeInsertable, S:PartialEq + Eq + Hash {}

///We need to implement our own Eq because we have to ignore the weak parent ref
impl<P,S,C> Hash for TreeNode<P,S,C> where P: TreeNodeInsertable, C : TreeNodeInsertable, S:PartialEq + Eq + Hash {
	fn hash<H>(&self, state:&mut H) -> () where H : Hasher{
		self.p.upgrade().hash(state); //FIXME why does the second syntax not work on this line?
		(&self.s).hash(state);
	}
}

pub enum TreeNodeItem<P,S,C> where P : TreeNodeInsertable, S: TreeNodeCarryable {
	TN(TreeNode<P,Value<S>,C>),
	Leaf(String),
}

///Marker trait for anything that ops in to being the payload of a TreeNode
pub trait TreeNodeCarryable : PartialEq + Eq + Hash{}
///For now, only 
impl TreeNodeCarryable for String{}

#[derive(PartialEq, Eq, Hash)]
pub struct Value<V>(V);

type NodeName = Value<String>;

#[derive(PartialEq,Eq, Hash)]
enum RustSackValue {
	name(NodeName),
}

#[derive(PartialEq,Eq, Hash)]
enum RustSackParent{
	rs(RustSack),
	nonr(())
}

impl TreeNodeInsertable for RustSackParent{}

#[derive(PartialEq,Eq, Hash)]
enum RustSackChild{
	rs(RustSack),
	poro(()) //PORO == Plain Old RustSack Object, and is any non-RustSack that can be put in a RustSack
}

impl TreeNodeInsertable for RustSackChild{}

type Sack<P,S> = TreeNode<P,S, Any>;
//pub trait RustSackInsertable : TreeNodeInsertable + Sized{}

type RustSack = TreeNode<RustSackParent, Value<RustSackValue>, RustSackChild>;
//impl TreeNodeInsertable for RustSack{}
//impl TreeNodeInsertable for RustSackParent{}

impl RustSack {
//	fn new() -> Self {
//		
//	}
}



trait TreeNodeParent<C : TreeNodeInsertable> : TreeNodeInsertable{}

