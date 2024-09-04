use crate::calculations::*;
use serenity::builder::{
    CreateButton, CreateEmbed, CreateMessage, CreateSelectMenu, CreateSelectMenuKind,
    CreateSelectMenuOption,
};

pub fn primos() -> CreateMessage {
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
    // use our message builder to add the blessing menu as a component.
    builder = builder.select_menu(blessing_menu);

    // define abyss menu's format
    let abyss_menu = CreateSelectMenu::new(
        "Abyss_menu",
        CreateSelectMenuKind::String {
            options: vec![
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
