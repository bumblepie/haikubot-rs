use crate::{
    database,
    formatting::{format_haiku_embed, to_embed_data},
};
use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateEmbed},
    client::Context,
    model::interactions::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
};
use slash_helper::{ApplicationCommandInteractionHandler, Command, InvocationError, ParseError};

pub struct RandomHaikuCommand;
pub const RANDOM_HAIKU_COMMAND_NAME: &'static str = "randomhaiku";

#[async_trait]
impl Command for RandomHaikuCommand {
    fn parse(_command: &ApplicationCommandInteraction) -> Result<Self, ParseError> {
        Ok(Self)
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name(RANDOM_HAIKU_COMMAND_NAME)
            .description("Fetch a random haiku from this server")
    }
}

#[async_trait]
impl ApplicationCommandInteractionHandler for RandomHaikuCommand {
    async fn invoke(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<(), InvocationError> {
        let haiku_and_id = if let Some(server_id) = command.guild_id {
            let db_connection = database::establish_connection();
            database::get_random_haiku(server_id, &db_connection)
        } else {
            None
        };
        if let Some((id, haiku)) = haiku_and_id {
            let embed_data = to_embed_data(id, &haiku, ctx).await;
            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            let mut embed = CreateEmbed::default();
                            format_haiku_embed(embed_data, &mut embed);
                            message.add_embed(embed);
                            message
                        })
                })
                .await
                .expect("Failed to send haiku msg");
        }
        Ok(())
    }
}