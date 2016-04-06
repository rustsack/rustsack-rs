use std::hash::Hash;
use std::collections::LinkedList;
use std::rc::Rc;
use std::hash::Hasher;

#[derive(Clone, Debug)]
///I'm calling them Treenodes in order to remind myself of their dual nature.
pub struct TreeNode<P, S, C>
    where P: TreeNodeParent<S>,
          S: Sized + PartialEq + Eq + Hash + TreeNodeInsertable
{
    ///child->parent relationships are necessarily weak
    pub p: Rc<P>,
    ///A generic treenode can have any value, but the value type needs to be able to be put inside of a parent's hash
    pub s: Box<S>,
    ///This should ideally be a non-distributed immutable/persistent tree. 
    ///Will approximate O(log N) insert and retrieve with O(N) ones for now
    pub c: LinkedList<C>,
}


// type Instantiatable<S, C> = TreeNode<(), S, C>;
// trait TreeInstantiable<S, C> {
//    fn new() -> Self;
// }

// impl<S, C> TreeInstantiable<S, C> for TreeNode<(), S, C>
//    where S: PartialEq + Eq + Hash + TreeNodeInsertable
// {
//    fn new() -> Self {
//        unimplemented!()
//    }
// }

pub trait Instantiatable {
    fn new() -> Self;
}

///Anything that can be the child of a treenode has to be Hash-able
pub trait TreeNodeInsertable: Sized + PartialEq + Eq + Hash {}
impl<S> TreeNodeInsertable for S where S: PartialEq + Eq + Hash {}

///We need to implement our own PartialEq because we have to ignore the weak parent ref
impl<P, S, C> PartialEq for TreeNode<P, S, C>
    where P: TreeNodeInsertable + TreeNodeParent<P> + TreeNodeParent<S>,
          C: TreeNodeInsertable,
          S: PartialEq + Eq + Hash + TreeNodeInsertable
{
    fn eq(&self, other: &TreeNode<P, S, C>) -> bool {
        self.s == other.s && self.c == other.c
    }
}

///We need to implement our own Eq because we have to ignore the weak parent ref
impl<P, S, C> Eq for TreeNode<P, S, C>
    where P: TreeNodeInsertable + TreeNodeParent<P> + TreeNodeParent<S>,
          C: TreeNodeInsertable,
          S: PartialEq + Eq + Hash + TreeNodeInsertable
{
}

///We need to implement our own Hash because we have to ignore the weak parent ref
impl<P, S, C> Hash for TreeNode<P, S, C>
    where P: TreeNodeInsertable + TreeNodeParent<S>,
          C: TreeNodeInsertable,
          S: PartialEq + Eq + Hash + TreeNodeInsertable
{
    fn hash<H>(&self, state: &mut H) -> ()
        where H: Hasher
    {
        self.p.hash(state); //FIXME why does the second syntax not work on this line?
        (&self.s).hash(state);
    }
}

///A finite set of all types of things that can be put in a treenode
pub enum TreeNodeItem<P, S, C>
    where P: TreeNodeInsertable + TreeNodeParent<TreeNodeValue<S>>,
          S: TreeNodeCarryable,
          TreeNodeValue<S>: TreeNodeInsertable
{
    TN(TreeNode<P, TreeNodeValue<S>, C>),
    Leaf(TreeNode<P, TreeNodeValue<S>, ()>),
}

///An infinite set of all types of things that treenodes can carry
pub trait TreeNodeCarryable: PartialEq + Eq + Hash {}
pub type NilParent = ();

///For now, only strings 
impl TreeNodeCarryable for String {}

#[derive(PartialEq, Eq, Hash)]
pub struct TreeNodeValue<V>(V);

pub type NodeName = TreeNodeValue<String>;

pub trait TreeNodeParent<C: TreeNodeInsertable>: TreeNodeInsertable {
    fn insert_child(self, C) -> Self;
}

impl<C> TreeNodeParent<C> for ()
    where C: TreeNodeInsertable
{
    fn insert_child(self, child: C) -> Self {
       unimplemented!() //this shouldn't be implemented. type system error
    }
}

#[test]
fn can_instantiate() {
    let _: TreeNode<(), String, String> = TreeNode {
        p: Rc::new(()),
        s: Box::new("Hello".to_owned()),
        c: LinkedList::new(),
    };
}

//impl<P, S, C> Instantiatable for TreeNode<P, S, C>
//    where P: TreeNodeParent<S> + TreeNodeParent<String>,
//          S: PartialEq + Eq + TreeNodeInsertable
//{
//    fn new() -> TreeNode<P, S, C> {
//        TreeNode::new()
//    }
//}
