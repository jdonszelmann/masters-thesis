extern crate serde;

use serde::{Deserialize};
use self::serde::{Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Union<T0, T1>{A(T0), B(T1)}

impl<T0, T1> Union<T0, T1> {
    pub fn for0(value: T0) -> Self {
        Union::A(value)
    }

    pub fn for1(value: T1) -> Self {
        Union::B(value)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Union3<T0, T1, T2>{A(T0), B(T1), C(T2)}

impl<T0, T1, T2> Union3<T0, T1, T2> {
    pub fn for0(value: T0) -> Self {
        Union3::A(value)
    }

    pub fn for1(value: T1) -> Self {
        Union3::B(value)
    }

    pub fn for2(value: T2) -> Self {
        Union3::C(value)
    }
}
