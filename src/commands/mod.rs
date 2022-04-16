use self::{
    count::CountCommand, gethaiku::GetHaikuCommand, random::RandomHaikuCommand,
    search::SearchCommand, uptime::UptimeCommand,
};
use serenity::{
    client::Context, model::interactions::application_command::ApplicationCommandInteraction,
};
use slash_helper::{ApplicationCommandInteractionHandler, InvocationError, ParseError};
use slash_helper_macros::Commands;

pub mod count;
pub mod gethaiku;
pub mod random;
pub mod search;
pub mod uptime;

#[derive(Commands)]
pub enum Commands {
    Uptime(UptimeCommand),
    Count(CountCommand),
    GetHaiku(GetHaikuCommand),
    RandomHaiku(RandomHaikuCommand),
    Search(SearchCommand),
}
