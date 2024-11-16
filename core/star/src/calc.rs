use super::*;
use std::{fmt::Debug, ops};

#[derive(Gate, Back, Default, Debug)]
pub struct Calc<T> {
    value: Hub<T>,
    ops: Vec<Operation<T>>,
}

impl<T> Solve for Calc<T>
where
    T: 'static
        + Gather
        + ops::AddAssign<T>
        + ops::SubAssign<T>
        + ops::MulAssign<T>
        + ops::DivAssign<T>,
{
    type Base = T;
    async fn solve(&self) -> node::Result<T> {
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
        Ok(out.into())
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
pub struct CalcBuilder<T> {
    target: Calc<T>,
}

impl<T> CalcBuilder<T>
where
    T: 'static + Gather,
    Calc<T>: IntoGateHub,
{
    pub fn hub(self) -> graph::Result<Hub<<Calc<T> as IntoGateHub>::Base>> {
        self.target.hub()
    }
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, value: impl Into<Hub<T>>) -> Self {
        self.target.ops.push(Operation {
            value: value.into(),
            type_: OperationType::Add,
        });
        self
    }
    #[allow(clippy::should_implement_trait)]
    pub fn sub(mut self, value: impl Into<Hub<T>>) -> Self {
        self.target.ops.push(Operation {
            value: value.into(),
            type_: OperationType::Sub,
        });
        self
    }
    #[allow(clippy::should_implement_trait)]
    pub fn mul(mut self, value: impl Into<Hub<T>>) -> Self {
        self.target.ops.push(Operation {
            value: value.into(),
            type_: OperationType::Mul,
        });
        self
    }
    #[allow(clippy::should_implement_trait)]
    pub fn div(mut self, value: impl Into<Hub<T>>) -> Self {
        self.target.ops.push(Operation {
            value: value.into(),
            type_: OperationType::Div,
        });
        self
    }
}

pub trait MakeCalc<T> {
    fn calc(&self) -> CalcBuilder<T>;
}

impl<T> MakeCalc<T> for Hub<T>
where
    T: 'static
        + Gather
        + ops::AddAssign<T>
        + ops::SubAssign<T>
        + ops::MulAssign<T>
        + ops::DivAssign<T>,
{
    fn calc(&self) -> CalcBuilder<T> {
        CalcBuilder {
            target: Calc {
                value: self.clone(),
                ops: vec![],
            },
        }
    }
}
