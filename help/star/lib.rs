use derive_builder::Builder;
use graph::*;
use node_derive::*;
use std::{fmt::Debug, ops};

#[derive(Builder, Back, Gate, Debug)]
#[builder(pattern = "owned")]
pub struct Vector<T> {
    #[builder(setter(each(name = "field", into)))]
    fields: Vec<Hub<T>>,
}

impl<T> Solve for Vector<T>
where
    T: 'static + Clone + SendSync + Debug,
{
    type Base = Vec<T>;
    async fn solve(&self) -> graph::Result<Hub<Vec<T>>> {
        let mut vector = vec![];
        for field in &self.fields {
            vector.push(field.base().await?);
        }
        Ok(vector.into_leaf().hub())
    }
}

#[derive(Back, Debug)]
pub struct Arithmetic<T> {
    value: Hub<T>,
    ops: Vec<Operation<T>>,
}

impl<T> GateTag for Arithmetic<T> {}

impl<T> Solve for Arithmetic<T>
where
    T: 'static + Clone + SendSync + Debug
        + ops::AddAssign<T> + ops::SubAssign<T> + ops::MulAssign<T> + ops::DivAssign<T>
{
    type Base = T;
    async fn solve(&self) -> graph::Result<Hub<T>> {
        let mut out = self.value.base().await?;
        for op in &self.ops {
            let value = op.value.base().await?;
            match op.type_ {
                OperationType::Add => out += value,
                OperationType::Sub => out -= value,
                OperationType::Mul => out *= value,
                OperationType::Div => out /= value,
            }
        }
        Ok(out.into_leaf().into())
    }
}

#[derive(Debug)]
struct Operation<T> {
    value: Hub<T>,
    type_: OperationType,
}

impl<T: 'static + Clone + SendSync> Backed for Operation<T> {
    fn backed(&self, back: &Back) -> Result<Self> {
        Ok(Self {
            value: self.value.backed(back)?,
            type_: self.type_.clone(),
        })
    }
}

#[derive(Clone, Debug)]
enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
}

pub struct ArithmeticBuilder<T> {
    target: Arithmetic<T>
}

impl<T> ArithmeticBuilder<T> 
where
    T: 'static + Clone + SendSync + Debug,
    Arithmetic<T>: Solve + IntoGate,
{
    pub fn hub(self) -> graph::Result<Hub<<Arithmetic<T> as Solve>::Base>> {
        Ok(self.target.gate()?.into())
    }
    pub fn add(mut self, value: impl Into<Hub<T>>) -> Self {
        self.target.ops.push(Operation {
            value: value.into(),
            type_: OperationType::Add,
        });
        self
    }
    pub fn sub(mut self, value: impl Into<Hub<T>>) -> Self {
        self.target.ops.push(Operation {
            value: value.into(),
            type_: OperationType::Sub,
        });
        self
    }
    pub fn mul(mut self, value: impl Into<Hub<T>>) -> Self {
        self.target.ops.push(Operation {
            value: value.into(),
            type_: OperationType::Mul,
        });
        self
    }
    pub fn div(mut self, value: impl Into<Hub<T>>) -> Self {
        self.target.ops.push(Operation {
            value: value.into(),
            type_: OperationType::Div,
        });
        self
    }
}

pub trait MakeArithmetic<T> {
    fn calc(&self) -> ArithmeticBuilder<T>;
}

impl<T> MakeArithmetic<T> for Hub<T> 
where 
    T: 'static + Clone + SendSync + Debug
        + ops::AddAssign<T> + ops::SubAssign<T> + ops::MulAssign<T> + ops::DivAssign<T>,
{
    fn calc(&self) -> ArithmeticBuilder<T> {
        ArithmeticBuilder {
            target: Arithmetic {
                value: self.clone(),
                ops: vec![] 
            }
        }
    }
}


// TODO: make trait to make new Sum with other
#[derive(Builder, Back, Gate, Debug)]
#[builder(pattern = "owned")]
pub struct Sum<T> {
    #[builder(setter(each(name = "field", into)))]
    pub fields: Vec<Hub<T>>,
}

impl<T> Solve for Sum<T>
where
    T: 'static + Clone + SendSync + Debug + Default + ops::AddAssign<T>,
{
    type Base = T;
    async fn solve(&self) -> graph::Result<Hub<T>> {
        let mut sum = T::default();
        for field in &self.fields {
            sum += field.base().await?;
        }
        Ok(sum.into_leaf().into())
    }
}

// TODO: make trait on Hub<T: Number> to make new Divide
#[derive(Builder, Back, Gate, Debug)]
#[builder(pattern = "owned")]
pub struct Divide<T> {
    pub dividend: Hub<T>,
    pub divisor: Hub<T>,
}

impl<T> Solve for Divide<T>
where
    T: 'static + Clone + SendSync + Debug + Default + ops::Div<Output = T>,
{
    type Base = T;
    async fn solve(&self) -> graph::Result<Hub<T>> {
        let quotient = self.dividend.base().await? / self.divisor.base().await?;
        Ok(quotient.into_leaf().into())
    }
}

#[derive(Builder, Back, Gate, Debug)]
#[builder(pattern = "owned")]
pub struct Multiply<T> {
    #[builder(setter(each(name = "field", into)))]
    pub fields: Vec<Hub<T>>,
}

impl<T> Solve for Multiply<T>
where
    T: 'static + Clone + SendSync + Debug + Default + ops::MulAssign<T>,
{
    type Base = T;
    async fn solve(&self) -> graph::Result<Hub<T>> {
        let mut product = T::default();
        for field in &self.fields {
            product *= field.base().await?;
        }
        Ok(product.into_leaf().into())
    }
}






// impl<T> Adapt for Vector<T>
// where
//     T: 'static + Clone + SendSync + Debug,
// {
//     fn back(&mut self, back: &Back) -> graph::Result<()> {
//         self.fields.back(back)
//     }
// }

// #[derive(Builder, Adapt)]
// #[builder(pattern = "owned")]
// pub struct Matrix {
//     #[builder(setter(each(name = "vector", into)))]
//     vectors: Vec<Hub<Vec<f64>>>,
// }

// impl Solve for Matrix {
//     type Base = Vec<f64>;
//     async fn solve(&self) -> Result<Hub<Vec<f64>>> {
//         let mut matrix = vec![];
//         for vector in &self.vectors {
//             matrix.extend(vector.base().await?);
//         }
//         Ok(matrix.into_leaf().hub())
//     }
//     fn rank(&self) -> u16 {
//         1
//     }
// }

// #[derive(Builder, Adapt, Gate, Debug)]
// #[builder(pattern = "owned")]
// pub struct Vector<T> {
//     #[builder(setter(each(name = "field", into)))]
//     fields: Vec<Hub<T>>,
// }

// impl<T> Solve for Vector<T> {
//     type Base = Vec<f64>;
//     async fn solve(&self) -> Result<Hub<Vec<f64>>> {
//         let mut vector = vec![];
//         for field in &self.fields {
//             vector.push(field.base().await?);
//         }
//         Ok(vector.into_leaf().hub())
//     }
//     fn rank(&self) -> u16 {
//         1
//     }
// }
