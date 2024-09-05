use crate::calculations::*;
use serenity::builder::{
    CreateButton, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
    CreateMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use std::time::Duration;
use serenity::futures::StreamExt;

pub fn howto() -> CreateMessage {
    let instructions = CreateEmbed::new().title("How to use Primobot").description(
        "Using command '*primos' will spawn an interactive menu which can be used to specify different parameters that are necessary for the estimation of the attainable amount of primogems.
These parameters include: 
- Blessing: If you have acquired the welking moon blessing or not.
- Abyss chambers: The amount of Abyss chambers that you are confident you will 3 star during each rotation.
- Imaginarium Theater stages: The amount of Imaginarium Theater stages you are able to complete each rotation.
- Days: The amount of days for which you want to estimate primogem gains.");
    return CreateMessage::new().add_embed(instructions);
}

pub fn primos_menu_message() -> CreateMessage {
    let embed = CreateEmbed::new().title("Primogem Estimator").description(
        "Estimate the minimum amount of primogems you will get in a certain amount of days",
    );
    // create a message builder that will let us add components before we send it.
    let mut builder = CreateMessage::new().add_embed(embed);
    // define blessing menu's format
    let blessing_menu = CreateSelectMenu::new(
        "Blessing_menu",
        CreateSelectMenuKind::String {
            options: vec![
                CreateSelectMenuOption::new("Blessing", "1"),
                CreateSelectMenuOption::new("No Blessing", "0"),
            ],
        },
    )
    .min_values(1)
    .max_values(1)
    .placeholder("Welkin Moon");
    // use the message builder to add the blessing menu as a component.
    builder = builder.select_menu(blessing_menu);

    // define abyss menu's format
    let abyss_menu = CreateSelectMenu::new(
        "Abyss_menu",
        CreateSelectMenuKind::String {
            options: vec![
                CreateSelectMenuOption::new("0", "0"),
                CreateSelectMenuOption::new("1", "1"),
                CreateSelectMenuOption::new("2", "2"),
                CreateSelectMenuOption::new("3", "3"),
                CreateSelectMenuOption::new("4", "4"),
                CreateSelectMenuOption::new("5", "5"),
                CreateSelectMenuOption::new("6", "6"),
                CreateSelectMenuOption::new("7", "7"),
                CreateSelectMenuOption::new("8", "8"),
                CreateSelectMenuOption::new("9", "9"),
                CreateSelectMenuOption::new("10", "10"),
                CreateSelectMenuOption::new("11", "11"),
                CreateSelectMenuOption::new("12", "12"),
            ],
        },
    )
    .custom_id("completed_abyss_chambers")
    .min_values(1)
    .max_values(1)
    .placeholder("3 star Abyss Chambers");
    // use the builder to add the abyss menu as a component.
    builder = builder.select_menu(abyss_menu);
    //define Imaginarium theater menu's format.
    let im_theater_menu = CreateSelectMenu::new(
        "Imaginarium_theater_menu",
        CreateSelectMenuKind::String {
            options: vec![
                CreateSelectMenuOption::new("0", "0"),
                CreateSelectMenuOption::new("1", "1"),
                CreateSelectMenuOption::new("2", "2"),
                CreateSelectMenuOption::new("3", "3"),
                CreateSelectMenuOption::new("4", "4"),
                CreateSelectMenuOption::new("5", "5"),
                CreateSelectMenuOption::new("6", "6"),
                CreateSelectMenuOption::new("7", "7"),
                CreateSelectMenuOption::new("8", "8"),
                CreateSelectMenuOption::new("9", "9"),
                CreateSelectMenuOption::new("10", "10"),
            ],
        },
    )
    .custom_id("Completed_IT_stages")
    .min_values(1)
    .max_values(1)
    .placeholder("Imaginarium Theater stages completed");
    // use the builder to add the Imaginarium Theater menu as a component.
    builder = builder.select_menu(im_theater_menu);
    return builder;
}

// This function will handle the user's choices previously made in primos_menu_message().
// It will also return a new create message with the primogem estimation result embeded in it.
pub async fn primos_handle_interactions(message: Message, context: Context) -> CreateMessage {
    // Waiting for the user to interact with the menu
    let mut interaction_stream = message
        .await_component_interaction(&context.shard)
        .timeout(Duration::from_secs(60 * 3))
        .stream();

    while let Some(interaction) = interaction_stream.next().await {
        let menu_values = &interaction.data.custom_id;
        // Acknowledge the interaction and send a reply
        interaction
            .create_response(
                &context,
                // This time we dont edit the message but reply to it
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::default()
                        // Make the message hidden for other users by setting `ephemeral(true)`.
                        .ephemeral(true)
                        .content(format!("response")),
                ),
            )
            .await
            .unwrap();
    }

    // Delete the orig message or there will be dangling components (components that still
    // exist, but no collector is running so any user who presses them sees an error)
    message.delete(&context).await.unwrap();
    let embed = CreateEmbed::new().title("hola").description("response1");
    return CreateMessage::new().add_embed(embed);
}
