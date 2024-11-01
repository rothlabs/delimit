use super::*;
use std::{fmt::Debug, ops};

#[derive(Back, Default, Debug)]
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

#[derive(Default)]
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