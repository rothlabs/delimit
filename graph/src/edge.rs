#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock};
#[cfg(feature = "oneThread")]
use std::{cell::RefCell, rc::Rc};

use crate::*;

/// Edge to a load.
pub type Ace<L> = Edge<apex::Ace<L>>;

/// Edge to a unit that grants a load.
pub type Agent<U> = Edge<apex::Agent<U>>;

/// The forward bridge between nodes.
pub struct Edge<N> {
    pub meta: Meta,
    pub back: Option<Back>,
    #[cfg(not(feature = "oneThread"))]
    pub apex: Arc<RwLock<N>>,
    #[cfg(feature = "oneThread")]
    pub apex: Rc<RefCell<N>>,
}

impl<N> ToMeta for Edge<N> {
    fn meta(&self) -> Meta {
        self.meta.clone()
    }
}

impl<N> FromItem for Edge<N>
where
    N: FromItem + ToMeta,
{
    type Item = N::Item;
    fn new(unit: Self::Item) -> Self {
        let apex = N::new(unit);
        Self {
            meta: apex.meta(),
            back: None,
            #[cfg(not(feature = "oneThread"))]
            apex: Arc::new(RwLock::new(apex)),
            #[cfg(feature = "oneThread")]
            apex: Rc::new(RefCell::new(apex)),
        }
    }
}

impl<N> Make for Edge<N>
where
    N: 'static + Default + DoMake + DoUpdate + ToMeta,
{
    type Unit = N::Unit;
    #[cfg(not(feature = "oneThread"))]
    fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self {
        let apex = N::default();
        let meta = apex.meta();
        let apex = Arc::new(RwLock::new(apex));
        let update = apex.clone() as Arc<RwLock<dyn DoUpdate>>;
        let back = Back::new(Arc::downgrade(&update));
        write_part(&apex, |mut apex| apex.do_make(make, &back));
        Self {
            meta,
            back: None,
            apex,
        }
    }
    #[cfg(feature = "oneThread")]
    fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self {
        let apex = N::default();
        let meta = apex.meta();
        let apex = Rc::new(RefCell::new(apex));
        let update = apex.clone() as Rc<RefCell<dyn DoUpdate>>;
        let back = Back::new(Rc::downgrade(&update));
        write_part(&apex, |mut apex| apex.do_make(make, &back));
        Self {
            meta,
            apex,
            back: None,
        }
    }
}

impl<N> Edge<N>
where
    N: 'static + DoUpdate,
{
    #[cfg(not(feature = "oneThread"))]
    fn node_as_back(&self) -> Back {
        let update = self.apex.clone() as Arc<RwLock<dyn DoUpdate>>;
        Back::new(Arc::downgrade(&update))
    }
    #[cfg(feature = "oneThread")]
    fn node_as_back(&self) -> Back {
        let update = self.apex.clone() as Rc<RefCell<dyn DoUpdate>>;
        Back::new(Rc::downgrade(&update))
    }
}

impl<N> Grant for Edge<N>
where
    N: 'static + DoGrant + DoUpdate,
{
    type Load = N::Load;
    fn grant(&self) -> Self::Load {
        write_part(&self.apex, |mut apex| apex.do_grant(&self.node_as_back()))
    }
}

impl<N> Insert for Edge<N> 
where 
    N: InsertMut
{
    fn insert(&self, field: &str, node: Node) {
        write_part(&self.apex, |mut apex| apex.insert_mut(field, node));
    }
}

impl<U, L> Produce<L> for Agent<U>
where
    U: 'static + Grant<Load = L> + SendSync,
    L: 'static + Clone + SendSync,
{
}

impl<U> ToPloy for Agent<U>
where
    U: 'static + Grant + SendSync,
    U::Load: 'static + Clone + SendSync,
{
    type Load = U::Load;
    #[cfg(not(feature = "oneThread"))]
    fn ploy(&self) -> Arc<RwLock<Box<dyn Produce<Self::Load>>>> {
        Arc::new(RwLock::new(Box::new(Self {
            meta: self.meta(),
            back: self.back.clone(),
            apex: self.apex.clone(),
        })))
    }
    #[cfg(feature = "oneThread")]
    fn ploy(&self) -> Rc<RefCell<Box<dyn Produce<Self::Load>>>> {
        //  + Send + Sync
        Rc::new(RefCell::new(Box::new(Self {
            meta: self.meta(),
            back: self.back.clone(),
            apex: self.apex.clone(),
        })))
    }
}

impl<U> BackedPloy for Agent<U>
where
    U: 'static + Grant + SendSync,
    U::Load: 'static + Clone + SendSync,
{
    type Load = U::Load;
    #[cfg(not(feature = "oneThread"))]
    fn backed_ploy(&self, back: &Back) -> Arc<RwLock<BoxProduce<Self::Load>>> {
        Arc::new(RwLock::new(Box::new(Self {
            meta: self.meta(),
            back: Some(back.clone()),
            apex: self.apex.clone(),
        })))
    }
    #[cfg(feature = "oneThread")]
    fn backed_ploy(&self, back: &Back) -> Rc<RefCell<BoxProduce<Self::Load>>> {
        Rc::new(RefCell::new(Box::new(Self {
            meta: self.meta(),
            back: Some(back.clone()),
            apex: self.apex.clone(),
        })))
    }
}

impl<N> ToLoad for Edge<N>
where
    N: ToLoad,
{
    type Load = N::Load;
    fn load(&self) -> Self::Load {
        read_part(&self.apex, |apex| apex.load())
    }
}

impl<N> Backed for Edge<N> {
    fn backed(&self, back: &Back) -> Self {
        Self {
            meta: self.meta(),
            back: Some(back.clone()),
            apex: self.apex.clone(),
        }
    }
}

impl<N> WriteLoad for Edge<N>
where
    N: WriteLoadOut,
{
    type Item = N::Item;
    fn write<T, F: FnOnce(&mut Self::Item) -> T>(&self, write: F) -> write::Result<T> {
        let write::Out { roots, meta, out } =
            write_part(&self.apex, |mut apex| apex.write_load_out(write));
        for root in &roots {
            root.react(&meta)?;
        }
        Ok(out)
    }
}

impl<N> WriteUnit for Edge<N>
where
    N: 'static + WriteUnitOut + DoUpdate,
{
    type Unit = N::Unit;
    fn write<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(&self, write: F) -> write::Result<T> {
        let write::Out { roots, meta, out } = write_part(&self.apex, |mut apex| {
            apex.write_unit_out(write, &self.node_as_back())
        });
        for root in &roots {
            root.react(&meta)?;
        }
        Ok(out)
    }
}

impl<N> Read for Edge<N>
where
    N: DoRead,
{
    type Item = N::Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        read_part(&self.apex, |apex| read(apex.do_read()))
    }
}

impl<N> AddRoot for Edge<N>
where
    N: DoAddRoot,
{
    fn add_root(&self, root: Root) {
        write_part(&self.apex, |mut apex| apex.do_add_root(root));
    }
}

impl<N> Update for Edge<N> where N: SendSync + DoReact {}

impl<N> Rebut for Edge<N> {
    fn rebut(&self) -> Ring {
        if let Some(back) = &self.back {
            back.rebut()
        } else {
            Ring::new()
        }
    }
}

impl<N> React for Edge<N>
where
    N: DoReact,
{
    fn react(&self, meta: &Meta) -> react::Result {
        write_part(&self.apex, |mut apex| apex.do_react(meta))
    }
}

type BoxProduce<L> = Box<dyn Produce<L>>;

impl<L> Grant for BoxProduce<L> {
    type Load = L;
    fn grant(&self) -> Self::Load {
        self.as_ref().grant()
    }
}

impl<L> AddRoot for BoxProduce<L> {
    fn add_root(&self, root: Root) {
        self.as_ref().add_root(root)
    }
}

impl<L> Update for BoxProduce<L> {}

impl<L> Rebut for BoxProduce<L> {
    fn rebut(&self) -> Ring {
        self.as_ref().rebut()
    }
}

impl<L> React for BoxProduce<L> {
    fn react(&self, meta: &Meta) -> react::Result {
        self.as_ref().react(meta)
    }
}



// impl<R, St> Serialize for Edge<R, St> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         self.meta.serialize(serializer)
//     }
// }

// impl<N> Write for Edge<N>
// where
//     N: DoWrite,
// {
//     type Item = N::Item;
//     fn write<T, F: FnOnce(&mut Self::Item) -> T>(&self, write: F) -> T {
//         write_part(&self.apex, |mut apex| apex.do_write(write))
//     }
// }
