//! Internal types used by command traits.
//!
//! This module contains types used by trait definitions in the [`command`]
//! module and implementations generated by the derive macros.
//!
//! [`command`]: crate::command

use std::collections::HashMap;

use twilight_model::{
    application::command::{
        BaseCommandOptionData, ChannelCommandOptionData, ChoiceCommandOptionData,
        CommandOptionChoice, CommandOptionValue, NumberCommandOptionData,
    },
    channel::ChannelType,
};

/// Convert a type to [`HashMap<String, String>`].
///
/// This method is used for the `name_localizations` and
/// `description_localizations` fields in macros implementations.
pub fn convert_localizations<I, K, V>(item: I) -> HashMap<String, String>
where
    I: IntoIterator<Item = (K, V)>,
    K: ToString,
    V: ToString,
{
    item.into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// Data to create a command option from.
///
/// This type is used in the [`CreateOption`] trait.
///
/// [`CreateOption`]: super::CreateOption
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateOptionData {
    /// Name of the option. It must be 32 characters or less.
    pub name: String,
    /// Localization dictionary for the option name. Keys must be valid locales.
    pub name_localizations: Option<HashMap<String, String>>,
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// Localization dictionary for the option description. Keys must be valid
    /// locales.
    pub description_localizations: Option<HashMap<String, String>>,
    /// Whether the option is required to be completed by a user.
    pub required: bool,
    /// Whether the command supports autocomplete. Only for `STRING`, `INTEGER`
    /// and `NUMBER` option types.
    pub autocomplete: bool,
    /// Data of the command option.
    pub data: CommandOptionData,
}

/// Data of a command option.
///
/// This type holds settings of a command option used when
/// parsing the option. It is used in the [`CommandOption`]
/// trait.
///
/// [`CommandOption`]: super::CommandOption
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommandOptionData {
    /// Restricts the channel choice to specific types. Only for `CHANNEL`
    /// option type.
    pub channel_types: Vec<ChannelType>,
    /// Maximum value permitted. Only for `INTEGER` and `NUMBER` option types.
    pub max_value: Option<CommandOptionValue>,
    /// Minimum value permitted. Only for `INTEGER` and `NUMBER` option types.
    pub min_value: Option<CommandOptionValue>,
}

impl CreateOptionData {
    /// Conversion into [`BaseCommandOptionData`]
    pub fn into_data(self) -> BaseCommandOptionData {
        BaseCommandOptionData {
            description: self.description,
            description_localizations: self.description_localizations,
            name: self.name,
            name_localizations: self.name_localizations,
            required: self.required,
        }
    }

    /// Conversion into [`ChannelCommandOptionData`]
    pub fn into_channel(self) -> ChannelCommandOptionData {
        ChannelCommandOptionData {
            channel_types: self.data.channel_types,
            description: self.description,
            description_localizations: self.description_localizations,
            name: self.name,
            name_localizations: self.name_localizations,
            required: self.required,
        }
    }

    /// Conversion into [`ChoiceCommandOptionData`]
    pub fn into_choice(self, choices: Vec<CommandOptionChoice>) -> ChoiceCommandOptionData {
        ChoiceCommandOptionData {
            autocomplete: self.autocomplete,
            choices,
            description: self.description,
            description_localizations: self.description_localizations,
            name: self.name,
            name_localizations: self.name_localizations,
            required: self.required,
        }
    }

    /// Conversion into [`NumberCommandOptionData`]
    pub fn into_number(self, choices: Vec<CommandOptionChoice>) -> NumberCommandOptionData {
        NumberCommandOptionData {
            autocomplete: self.autocomplete,
            choices,
            description: self.description,
            description_localizations: self.description_localizations,
            max_value: self.data.max_value,
            min_value: self.data.min_value,
            name: self.name,
            name_localizations: self.name_localizations,
            required: self.required,
        }
    }
}
