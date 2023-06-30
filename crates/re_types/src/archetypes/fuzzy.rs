// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_cast)]

#[derive(Debug, Clone, PartialEq)]
pub struct AffixFuzzer1 {
    pub fuzz1001: crate::components::AffixFuzzer1,
    pub fuzz1002: crate::components::AffixFuzzer2,
    pub fuzz1003: crate::components::AffixFuzzer3,
    pub fuzz1004: crate::components::AffixFuzzer4,
    pub fuzz1005: crate::components::AffixFuzzer5,
    pub fuzz1006: crate::components::AffixFuzzer6,
    pub fuzz1007: crate::components::AffixFuzzer7,
    pub fuzz1101: Vec<crate::components::AffixFuzzer1>,
    pub fuzz1102: Vec<crate::components::AffixFuzzer2>,
    pub fuzz1103: Vec<crate::components::AffixFuzzer3>,
    pub fuzz1104: Vec<crate::components::AffixFuzzer4>,
    pub fuzz1105: Vec<crate::components::AffixFuzzer5>,
    pub fuzz1106: Vec<crate::components::AffixFuzzer6>,
    pub fuzz1107: Vec<crate::components::AffixFuzzer7>,
    pub fuzz2001: Option<crate::components::AffixFuzzer1>,
    pub fuzz2002: Option<crate::components::AffixFuzzer2>,
    pub fuzz2003: Option<crate::components::AffixFuzzer3>,
    pub fuzz2004: Option<crate::components::AffixFuzzer4>,
    pub fuzz2005: Option<crate::components::AffixFuzzer5>,
    pub fuzz2006: Option<crate::components::AffixFuzzer6>,
    pub fuzz2007: Option<crate::components::AffixFuzzer7>,
    pub fuzz2101: Option<Vec<crate::components::AffixFuzzer1>>,
    pub fuzz2102: Option<Vec<crate::components::AffixFuzzer2>>,
    pub fuzz2103: Option<Vec<crate::components::AffixFuzzer3>>,
    pub fuzz2104: Option<Vec<crate::components::AffixFuzzer4>>,
    pub fuzz2105: Option<Vec<crate::components::AffixFuzzer5>>,
    pub fuzz2106: Option<Vec<crate::components::AffixFuzzer6>>,
    pub fuzz2107: Option<Vec<crate::components::AffixFuzzer7>>,
}

impl AffixFuzzer1 {
    pub const REQUIRED_COMPONENTS: [crate::ComponentName; 14usize] = [
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer1"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer2"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer3"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer4"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer5"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer6"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer7"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer1"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer2"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer3"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer4"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer5"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer6"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer7"),
    ];

    pub const RECOMMENDED_COMPONENTS: [crate::ComponentName; 0usize] = [];

    pub const OPTIONAL_COMPONENTS: [crate::ComponentName; 14usize] = [
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer1"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer2"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer3"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer4"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer5"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer6"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer7"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer1"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer2"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer3"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer4"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer5"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer6"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer7"),
    ];

    pub const ALL_COMPONENTS: [crate::ComponentName; 28usize] = [
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer1"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer2"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer3"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer4"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer5"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer6"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer7"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer1"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer2"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer3"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer4"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer5"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer6"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer7"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer1"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer2"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer3"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer4"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer5"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer6"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer7"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer1"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer2"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer3"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer4"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer5"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer6"),
        crate::ComponentName::Borrowed("rerun.testing.components.AffixFuzzer7"),
    ];
}

impl crate::Archetype for AffixFuzzer1 {
    fn name() -> crate::ArchetypeName {
        crate::ArchetypeName::Borrowed("rerun.testing.archetypes.AffixFuzzer1")
    }

    fn required_components() -> Vec<crate::ComponentName> {
        Self::REQUIRED_COMPONENTS.to_vec()
    }

    fn recommended_components() -> Vec<crate::ComponentName> {
        Self::RECOMMENDED_COMPONENTS.to_vec()
    }

    fn optional_components() -> Vec<crate::ComponentName> {
        Self::OPTIONAL_COMPONENTS.to_vec()
    }

    #[allow(clippy::todo)]
    fn to_arrow_datatypes() -> Vec<arrow2::datatypes::DataType> {
        todo!("query the registry for all fqnames");
    }
}

impl AffixFuzzer1 {
    pub fn new(
        fuzz1001: impl Into<crate::components::AffixFuzzer1>,
        fuzz1002: impl Into<crate::components::AffixFuzzer2>,
        fuzz1003: impl Into<crate::components::AffixFuzzer3>,
        fuzz1004: impl Into<crate::components::AffixFuzzer4>,
        fuzz1005: impl Into<crate::components::AffixFuzzer5>,
        fuzz1006: impl Into<crate::components::AffixFuzzer6>,
        fuzz1007: impl Into<crate::components::AffixFuzzer7>,
        fuzz1101: impl IntoIterator<Item = impl Into<crate::components::AffixFuzzer1>>,
        fuzz1102: impl IntoIterator<Item = impl Into<crate::components::AffixFuzzer2>>,
        fuzz1103: impl IntoIterator<Item = impl Into<crate::components::AffixFuzzer3>>,
        fuzz1104: impl IntoIterator<Item = impl Into<crate::components::AffixFuzzer4>>,
        fuzz1105: impl IntoIterator<Item = impl Into<crate::components::AffixFuzzer5>>,
        fuzz1106: impl IntoIterator<Item = impl Into<crate::components::AffixFuzzer6>>,
        fuzz1107: impl IntoIterator<Item = impl Into<crate::components::AffixFuzzer7>>,
    ) -> Self {
        Self {
            fuzz1001: fuzz1001.into(),
            fuzz1002: fuzz1002.into(),
            fuzz1003: fuzz1003.into(),
            fuzz1004: fuzz1004.into(),
            fuzz1005: fuzz1005.into(),
            fuzz1006: fuzz1006.into(),
            fuzz1007: fuzz1007.into(),
            fuzz1101: fuzz1101.into_iter().map(Into::into).collect(),
            fuzz1102: fuzz1102.into_iter().map(Into::into).collect(),
            fuzz1103: fuzz1103.into_iter().map(Into::into).collect(),
            fuzz1104: fuzz1104.into_iter().map(Into::into).collect(),
            fuzz1105: fuzz1105.into_iter().map(Into::into).collect(),
            fuzz1106: fuzz1106.into_iter().map(Into::into).collect(),
            fuzz1107: fuzz1107.into_iter().map(Into::into).collect(),
            fuzz2001: None,
            fuzz2002: None,
            fuzz2003: None,
            fuzz2004: None,
            fuzz2005: None,
            fuzz2006: None,
            fuzz2007: None,
            fuzz2101: None,
            fuzz2102: None,
            fuzz2103: None,
            fuzz2104: None,
            fuzz2105: None,
            fuzz2106: None,
            fuzz2107: None,
        }
    }

    pub fn with_fuzz2001(mut self, fuzz2001: impl Into<crate::components::AffixFuzzer1>) -> Self {
        self.fuzz2001 = Some(fuzz2001.into());
        self
    }

    pub fn with_fuzz2002(mut self, fuzz2002: impl Into<crate::components::AffixFuzzer2>) -> Self {
        self.fuzz2002 = Some(fuzz2002.into());
        self
    }

    pub fn with_fuzz2003(mut self, fuzz2003: impl Into<crate::components::AffixFuzzer3>) -> Self {
        self.fuzz2003 = Some(fuzz2003.into());
        self
    }

    pub fn with_fuzz2004(mut self, fuzz2004: impl Into<crate::components::AffixFuzzer4>) -> Self {
        self.fuzz2004 = Some(fuzz2004.into());
        self
    }

    pub fn with_fuzz2005(mut self, fuzz2005: impl Into<crate::components::AffixFuzzer5>) -> Self {
        self.fuzz2005 = Some(fuzz2005.into());
        self
    }

    pub fn with_fuzz2006(mut self, fuzz2006: impl Into<crate::components::AffixFuzzer6>) -> Self {
        self.fuzz2006 = Some(fuzz2006.into());
        self
    }

    pub fn with_fuzz2007(mut self, fuzz2007: impl Into<crate::components::AffixFuzzer7>) -> Self {
        self.fuzz2007 = Some(fuzz2007.into());
        self
    }

    pub fn with_fuzz2101(
        mut self,
        fuzz2101: impl IntoIterator<Item = impl Into<crate::components::AffixFuzzer1>>,
    ) -> Self {
        self.fuzz2101 = Some(fuzz2101.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_fuzz2102(
        mut self,
        fuzz2102: impl IntoIterator<Item = impl Into<crate::components::AffixFuzzer2>>,
    ) -> Self {
        self.fuzz2102 = Some(fuzz2102.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_fuzz2103(
        mut self,
        fuzz2103: impl IntoIterator<Item = impl Into<crate::components::AffixFuzzer3>>,
    ) -> Self {
        self.fuzz2103 = Some(fuzz2103.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_fuzz2104(
        mut self,
        fuzz2104: impl IntoIterator<Item = impl Into<crate::components::AffixFuzzer4>>,
    ) -> Self {
        self.fuzz2104 = Some(fuzz2104.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_fuzz2105(
        mut self,
        fuzz2105: impl IntoIterator<Item = impl Into<crate::components::AffixFuzzer5>>,
    ) -> Self {
        self.fuzz2105 = Some(fuzz2105.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_fuzz2106(
        mut self,
        fuzz2106: impl IntoIterator<Item = impl Into<crate::components::AffixFuzzer6>>,
    ) -> Self {
        self.fuzz2106 = Some(fuzz2106.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_fuzz2107(
        mut self,
        fuzz2107: impl IntoIterator<Item = impl Into<crate::components::AffixFuzzer7>>,
    ) -> Self {
        self.fuzz2107 = Some(fuzz2107.into_iter().map(Into::into).collect());
        self
    }
}
