//! Internal types used by command traits.
//!
//! This module contains types used in traits definitions of the [`command`] module
//! and used by implementations generated by the derive macros.
//!
//! <pre class="compile_fail" style="white-space:normal;font:inherit;">
//!     <strong>Warning</strong>: Types exposed by this modules are not intended
//!     to be used directly and do not respect semantic versioning. Breaking
//!     changes may occur between minor version.
//! </pre>
//!
//! [`command`]: crate::command

use twilight_model::{
    application::command::{
        BaseCommandOptionData, ChannelCommandOptionData, ChoiceCommandOptionData,
        CommandOptionChoice, CommandOptionValue, NumberCommandOptionData,
    },
    channel::ChannelType,
};

/// Data to create a command option.
///
/// This type is used in the [`CreateOption`] trait.
///
/// <pre class="compile_fail" style="white-space:normal;font:inherit;">
///     <strong>Warning</strong>: This type is not intended to be used directly
///     and does not respect semantic versioning. New fields can be introduced
///     between minor versions.
/// </pre>
///
/// [`CreateOption`]: super::CreateOption
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateOptionData {
    /// Name of the option. It must be 32 characters or less.
    pub name: String,
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// Whether the option is required to be completed by a user.
    pub required: bool,
    /// Whether the command supports autocomplete. Only for `STRING`, `INTEGER` and `NUMBER` option type.
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
/// <pre class="compile_fail" style="white-space:normal;font:inherit;">
///     <strong>Warning</strong>: This type is not intended to be used directly
///     and does not respect semantic versioning. New fields can be introduced
///     between minor versions.
/// </pre>
///
/// [`CommandOption`]: super::CommandOption
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommandOptionData {
    /// Restricts the channel choice to specific types. Only for `CHANNEL` option type.
    pub channel_types: Vec<ChannelType>,
    /// Maximum value permitted. Only for `INTEGER` and `NUMBER` option type.
    pub max_value: Option<CommandOptionValue>,
    /// Minimum value permitted. Only for `INTEGER` and `NUMBER` option type.
    pub min_value: Option<CommandOptionValue>,
}

impl CreateOptionData {
    /// Conversion into a [`BaseCommandOptionData`]
    pub fn into_data(self) -> BaseCommandOptionData {
        BaseCommandOptionData {
            description: self.description,
            name: self.name,
            required: self.required,
        }
    }

    /// Conversion into a [`ChannelCommandOptionData`]
    pub fn into_channel(self) -> ChannelCommandOptionData {
        ChannelCommandOptionData {
            channel_types: self.data.channel_types,
            description: self.description,
            name: self.name,
            required: self.required,
        }
    }

    /// Conversion into a [`ChoiceCommandOptionData`]
    pub fn into_choice(self, choices: Vec<CommandOptionChoice>) -> ChoiceCommandOptionData {
        ChoiceCommandOptionData {
            autocomplete: self.autocomplete,
            choices,
            description: self.description,
            name: self.name,
            required: self.required,
        }
    }

    /// Conversion into a [`NumberCommandOptionData`]
    pub fn into_number(self, choices: Vec<CommandOptionChoice>) -> NumberCommandOptionData {
        NumberCommandOptionData {
            autocomplete: self.autocomplete,
            choices,
            description: self.description,
            max_value: self.data.max_value,
            min_value: self.data.min_value,
            name: self.name,
            required: self.required,
        }
    }
}