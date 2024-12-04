use super::*;
use std::{fmt::Debug, ops};

#[derive(Gate, Back, Default, Debug)]
pub struct Math<T> {
    value: Hub<T>,
    ops: Vec<Operation<T>>,
}

impl<T> Solve for Math<T>
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
                // OperationType::DivUp => out = (out + value - 1) / value,
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
    fn backed(&self, back: &Back) -> Self {
        Self {
            value: self.value.backed(back),
            type_: self.type_.clone(),
        }
    }
}

#[derive(Clone, Debug)]
enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
    // DivUp,
}

#[derive(Default)]
pub struct MathBuilder<T> {
    target: Math<T>,
}

impl<T> MathBuilder<T>
where
    T: 'static + Gather,
    Math<T>: IntoGateHub,
{
    pub fn hub(self) -> Hub<<Math<T> as IntoGateHub>::Base> {
        self.target.hub()
        // let wow = self.target.gate()?;
        // Ok(wow.into())
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

pub trait Calculate<T> {
    fn math(&self) -> MathBuilder<T>;
}

impl<T> Calculate<T> for Hub<T>
where
    T: 'static
        + Gather
        + ops::AddAssign<T>
        + ops::SubAssign<T>
        + ops::MulAssign<T>
        + ops::DivAssign<T>,
{
    fn math(&self) -> MathBuilder<T> {
        MathBuilder {
            target: Math {
                value: self.clone(),
                ops: vec![],
            },
        }
    }
}
