use flowy_derive::{ProtoBuf, ProtoBuf_Enum};

#[derive(Eq, PartialEq, ProtoBuf, Debug, Default, Clone)]
pub struct UrlGroupConfigurationPB {
    #[pb(index = 1)]
    hide_empty: bool,
}

#[derive(Eq, PartialEq, ProtoBuf, Debug, Default, Clone)]
pub struct TextGroupConfigurationPB {
    #[pb(index = 1)]
    hide_empty: bool,
}

#[derive(Eq, PartialEq, ProtoBuf, Debug, Default, Clone)]
pub struct SelectOptionGroupConfigurationPB {
    #[pb(index = 1)]
    hide_empty: bool,
}

#[derive(Eq, PartialEq, ProtoBuf, Debug, Default, Clone)]
pub struct NumberGroupConfigurationPB {
    #[pb(index = 1)]
    hide_empty: bool,
}

#[derive(Eq, PartialEq, ProtoBuf, Debug, Default, Clone)]
pub struct DateGroupConfigurationPB {
    #[pb(index = 1)]
    pub condition: DateCondition,

    #[pb(index = 2)]
    hide_empty: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, ProtoBuf_Enum)]
#[repr(u8)]
pub enum DateCondition {
    Relative = 0,
    Day = 1,
    Week = 2,
    Month = 3,
    Year = 4,
}

impl std::default::Default for DateCondition {
    fn default() -> Self {
        DateCondition::Relative
    }
}

#[derive(Eq, PartialEq, ProtoBuf, Debug, Default, Clone)]
pub struct CheckboxGroupConfigurationPB {
    #[pb(index = 1)]
    pub(crate) hide_empty: bool,
}
