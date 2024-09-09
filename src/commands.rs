use crate::calculations::{self};
use serenity::all::{ButtonStyle, ChannelId, ComponentInteractionDataKind, EditMessage};
use serenity::builder::{
    CreateButton, CreateEmbed, CreateInteractionResponse, CreateMessage, CreateSelectMenu,
    CreateSelectMenuKind, CreateSelectMenuOption,
};
use serenity::http::CacheHttp;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use std::time::Duration;

// This function updates the message sent by the bot to display the users desired amount of days.
async fn edit_days_in_message(
    bot_message: &mut Message,
    context: &Context,
    days: u32,
) -> Result<(), serenity::prelude::SerenityError> {
    let new_embed = if days > 1 {
        CreateEmbed::new()
            .title("Primogem Estimator")
            .description(format!(
                "Estimate the minimun amount of primogems you will get in: __**{} days**__",
                days
            ))
    } else {
        CreateEmbed::new()
            .title("Primogem Estimator")
            .description("Estimate the minimun amount of primogems you will get in: __**1 day**__")
    };
    let builder = EditMessage::new().embed(new_embed);
    return bot_message.edit(context, builder).await;
}

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

fn get_menu_value(interaction: ComponentInteractionDataKind) -> u32 {
    let mut result: u32 = 0;
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
    let embed = CreateEmbed::new()
        .title("Primogem Estimator")
        .description("Estimate the minimun amount of primogems you will get in: __**1 day**__");
    let mut days = 1;

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
    let mut bot_message = match channel.send_message(context.http(), builder).await {
        Ok(sent_message) => sent_message,
        Err(error) => {
            println!("Encountered: {error} and couldnt send a message");
            return;
        }
    };

    // Interaction handling and value assignment.
    let mut abyss_chambers = 0;
    let mut theater_stages = 0;
    let mut blessing = false;
    let mut days: u32 = 1;

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
                let _ = bot_message.delete(&context).await;
                let title = if days > 1 {
                    format!("Your estimation for {} days", days)
                } else {
                    "Your estimation for 1 day".to_string()
                };
                let result_embed =
                    CreateEmbed::new()
                        .title(title)
                        .description(calculations::estimate_primogems(
                            days,
                            blessing,
                            abyss_chambers,
                            theater_stages,
                        ));
                let result_message = CreateMessage::new().add_embed(result_embed);
                let _ = channel.send_message(context.http(), result_message).await;
            }
            "+1" => {
                days += 1;
                match edit_days_in_message(&mut bot_message, context, days).await {
                    Ok(_) => (),
                    Err(error) => {
                        println!("Error: {error} occurred.\nHad to delete message");
                        bot_message.delete(&context).await.unwrap();
                    }
                }
            }
            "+10" => {
                days += 10;
                match edit_days_in_message(&mut bot_message, context, days).await {
                    Ok(_) => (),
                    Err(error) => {
                        println!("Error: {error} occurred.\nHad to delete message");
                        bot_message.delete(&context).await.unwrap();
                    }
                }
            }
            "-10" => {
                if days <= 10 {
                    days = 1;
                } else {
                    days -= 10;
                };
                match edit_days_in_message(&mut bot_message, context, days).await {
                    Ok(_) => (),
                    Err(error) => {
                        println!("Error: {error} occurred.\nHad to delete message");
                        bot_message.delete(&context).await.unwrap();
                    }
                }
            }
            "-1" => {
                if days > 1 {
                    days -= 1;
                };
                match edit_days_in_message(&mut bot_message, context, days).await {
                    Ok(_) => (),
                    Err(error) => {
                        println!("Error: {error} occurred.\nHad to delete message");
                        bot_message.delete(&context).await.unwrap();
                    }
                }
            }
            unexpected => {
                println!("There was an unexpected interaction: ({component}, {unexpected})")
            }
        }
    }
}
