use super::*;

//TODO: make special 'work' that uses a grantor_with_back and saves the base here
pub type Pipe<H> = Ace<Inner<H>>;

type Inner<H> = Ace<Unit<H>>;

struct Unit<H> 
where 
    H: Grant,
{
    high: H,
    base: H::Load,
}

impl<H> Grant for Unit<H> 
where 
    H: Grant,
{
    type Load = H::Load;
    fn grant(&self) -> Self::Load {
        self.high.grant()
    }
}




// pub type Pipe<H, L> = Deuce<Unit<H, Ploy<L>>, L>;

// pub struct Unit<H, B> {
//     high: H,
//     base: B,
// }

// impl<H, B> Grant for Unit<H, B> 
// where 
//     H: Grant<Load = B>,
//     B: Grant,
// {
//     type Load = B::Load;
//     fn grant(&self) -> Self::Load {
//         self.high.grant().grant()
//     }
// }



