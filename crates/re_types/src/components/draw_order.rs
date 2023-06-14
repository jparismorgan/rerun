// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

/// Draw order used for the display order of 2D elements.
///
/// Higher values are drawn on top of lower values.
/// An entity can have only a single draw order component.
/// Within an entity draw order is governed by the order of the components.
///
/// Draw order for entities with the same draw order is generally undefined.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DrawOrder(pub f32);

impl crate::Component for DrawOrder {
    fn name() -> crate::ComponentName {
        crate::ComponentName::Borrowed("rerun.components.DrawOrder")
    }

    #[allow(clippy::wildcard_imports)]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Float32
    }
}