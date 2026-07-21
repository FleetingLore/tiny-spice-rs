//! 基本元件

use std::fmt;

/// 电容
pub mod capacitor;

/// 二极管
pub mod diode;

/// 电阻
pub mod resistor;

/// 正弦电流源
pub mod isine;

/// 正弦电压源
pub mod vsine;

/// 分段线性电压源
pub mod vpwl;

/// 独立源
pub mod independent;

/// 依赖源
pub mod vdepsrc;

pub use crate::element::capacitor::Capacitor;
pub use crate::element::diode::Diode;
pub use crate::element::independent::CurrentSource;
pub use crate::element::independent::VoltageSource;
pub use crate::element::isine::CurrentSourceSine;
pub use crate::element::resistor::Resistor;
pub use crate::element::vdepsrc::{Vccs, Vcvs};
pub use crate::element::vpwl::VoltageSourcePwl;
pub use crate::element::vsine::VoltageSourceSine;

/// Circuit Elements that this simulator supports
#[allow(dead_code)]
#[derive(Clone)]
pub enum Element {
    // 电阻
    R(Resistor),

    // 电流源
    I(CurrentSource),

    // 电压源
    V(VoltageSource),

    // 二极管
    D(Diode),

    // 正弦电流源
    Isin(CurrentSourceSine),

    // 正弦电压源
    Vsin(VoltageSourceSine),

    // 分段线性电压源
    Vpwl(VoltageSourcePwl),

    // 电容
    C(Capacitor),

    // 电压控制电流源
    Vcvs(Vcvs),

    // 电压控制电压源
    Vccs(Vccs),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Element::R(el) => write!(f, "{el}"),
            Element::I(el) => write!(f, "{el}"),
            Element::V(el) => write!(f, "{el}"),
            Element::D(el) => write!(f, "{el}"),
            Element::Isin(el) => write!(f, "{el}"),
            Element::Vsin(el) => write!(f, "{el}"),
            Element::Vpwl(el) => write!(f, "{el}"),
            Element::C(el) => write!(f, "{el}"),
            Element::Vcvs(el) => write!(f, "{el}"),
            Element::Vccs(el) => write!(f, "{el}"),
        }
    }
}
