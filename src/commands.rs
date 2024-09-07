use crate::calculations::*;
use serenity::all::{ButtonStyle, ChannelId, ComponentInteractionDataKind, CreateChannel, EmojiId};
use serenity::builder::{
    CreateButton, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
    CreateMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
};
use serenity::futures::StreamExt;
use serenity::http::CacheHttp;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use std::time::Duration;

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

fn get_menu_value(interaction: ComponentInteractionDataKind) -> i32 {
    let mut result: i32 = 0;
    match interaction {
        ComponentInteractionDataKind::StringSelect { values, .. } => {
            result = values
                .first()
                .unwrap_or(&"0".to_string())
                .parse()
                .unwrap_or(0);
        }
        some_other_kind => {
            println!("Got an unexpected interaction kind: {some_other_kind:?}")
        }
    }
    result
}
pub async fn primos_menu_message(message: &Message, context: &Context) {
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
    .custom_id("Blessing_menu")
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
    .custom_id("Abyss_menu")
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
    .custom_id("Imaginarium_theater_menu")
    .min_values(1)
    .max_values(1)
    .placeholder("Imaginarium Theater stages completed");

    // use the builder to add the Imaginarium Theater menu as a component.
    builder = builder.select_menu(im_theater_menu);

    // add estimation button to the builder.
    builder = builder.button(
        CreateButton::new("Estimate")
            .style(ButtonStyle::Primary)
            .label("Estimate"),
    );

    // add day adding and subtraction buttons (+1 ,+10, -10 ,-1)
    builder = builder.button(
        CreateButton::new("+1")
            .style(ButtonStyle::Success)
            .label("+1 day"),
    );
    builder = builder.button(
        CreateButton::new("+10")
            .style(ButtonStyle::Success)
            .label("+10 days"),
    );
    builder = builder.button(
        CreateButton::new("-10")
            .style(ButtonStyle::Danger)
            .label("-10 days"),
    );
    builder = builder.button(
        CreateButton::new("-1")
            .style(ButtonStyle::Danger)
            .label("-1 day"),
    );

    let channel: ChannelId = message.channel_id;
    if let Err(error) = channel.send_message(context.http(), builder).await {
        println!("Encountered: {error}");
    }

    // Interaction handling and value assignment.
    let mut abyss_chambers = 0;
    let mut theater_stages = 0;
    let mut blessing = false;

    // Handle all incoming interactions with menus and buttons.
    while let Some(interaction) = serenity::collector::ComponentInteractionCollector::new(context)
        .timeout(Duration::from_secs(60))
        .await
    {
        // It IS NECESSARY to acknowledge the interaction being received, Otherwise users will get an
        // interaction failed error on your discord client
        let _acknowledge_interaction = interaction
            .create_response(context.http(), CreateInteractionResponse::Acknowledge)
            .await;
        let component = interaction.data.custom_id.as_str();
        match component {
            "Blessing_menu" => {
                if get_menu_value(interaction.data.kind) == 1 {
                    blessing = true;
                }
            }
            "Abyss_menu" => {
                abyss_chambers = get_menu_value(interaction.data.kind);
            }
            "Imaginarium_theater_menu" => {
                theater_stages = get_menu_value(interaction.data.kind);
            }
            "Estimate" => {
                println!("estimating")
            }
            "+1" => {}
            "+10" => {}
            "-10" => {}
            "-1" => {}
            unexpected => {
                println!("There was an unexpected interaction: ({component}, {unexpected})")
            }
        }
    }
    println!("after interaction reached");

    // Delete the orig message or there will be dangling components (components that still
    // exist, but no collector is running so any user who presses them sees an error)
    match message.delete(&context).await {
        Err(error) => {
            println!(
                "There was an error while trying to delete the original message due to {error}"
            )
        }
        Ok(_) => {
            println!("Original message should have been deleted by now")
        }
    }
}
