//! Displays the Cycle Count as text

use bevy::prelude::*;

use crate::game::spawn::atom::Electron;
use crate::ui::palette::BUTTON_TEXT;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_cycle_ui);
    app.add_systems(Update, update_atom_label_text);
}

#[derive(Event, Debug)]
pub struct SpawnAtomLabel;

fn spawn_cycle_ui(_trigger: Trigger<SpawnAtomLabel>, mut commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    // commands.trigger(SpawnPlayer);

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Cycles: ",
                TextStyle {
                    font_size: 40.0,
                    color: BUTTON_TEXT,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 40.0,
                color: BUTTON_TEXT,
                ..default()
            }),
        ]) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        AtomLabelText::new(),
    ));
}

#[derive(Component)]
pub struct AtomLabelText(usize);

impl AtomLabelText {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn label(&self) -> &'static str {
        match self.0 {
            0 => "",
            1 => "Hydrogen",
            2 => "Helium",
            3 => "Lithium",
            4 => "Beryllium",
            5 => "Boron",
            6 => "Carbon",
            7 => "Nitrogen",
            8 => "Oxygen",
            9 => "Fluorine",
            10 => "Neon",
            11 => "Sodium",
            12 => "Magnesium",
            13 => "Aluminum",
            14 => "Silicon",
            15 => "Phosphorus",
            16 => "Sulfur",
            17 => "Chlorine",
            18 => "Argon",
            19 => "Potassium",
            20 => "Calcium",
            21 => "Scandium",
            22 => "Titanium",
            23 => "Vanadium",
            24 => "Chromium",
            25 => "Manganese",
            26 => "Iron",
            27 => "Cobalt",
            28 => "Nickel",
            29 => "Copper",
            30 => "Zinc",
            31 => "Gallium",
            32 => "Germanium",
            33 => "Arsenic",
            34 => "Selenium",
            35 => "Bromine",
            36 => "Krypton",
            37 => "Rubidium",
            38 => "Strontium",
            39 => "Yttrium",
            40 => "Zirconium",
            41 => "Niobium",
            42 => "Molybdenum",
            43 => "Technetium",
            44 => "Ruthenium",
            45 => "Rhodium",
            46 => "Palladium",
            47 => "Silver",
            48 => "Cadmium",
            49 => "Indium",
            50 => "Tin",
            51 => "Antimony",
            52 => "Tellurium",
            53 => "Iodine",
            54 => "Xenon",
            55 => "Cesium",
            56 => "Barium",
            57 => "Lanthanum",
            58 => "Cerium",
            59 => "Praseodymium",
            60 => "Neodymium",
            61 => "Promethium",
            62 => "Samarium",
            63 => "Europium",
            64 => "Gadolinium",
            65 => "Terbium",
            66 => "Dysprosium",
            67 => "Holmium",
            68 => "Erbium",
            69 => "Thulium",
            70 => "Ytterbium",
            71 => "Lutetium",
            72 => "Hafnium",
            73 => "Tantalum",
            74 => "Wolfram",
            75 => "Rhenium",
            76 => "Osmium",
            77 => "Iridium",
            78 => "Platinum",
            79 => "Gold",
            80 => "Mercury",
            81 => "Thallium",
            82 => "Lead",
            83 => "Bismuth",
            84 => "Polonium",
            85 => "Astatine",
            86 => "Radon",
            87 => "Francium",
            88 => "Radium",
            89 => "Actinium",
            90 => "Thorium",
            91 => "Protactinium",
            92 => "Uranium",
            93 => "Neptunium",
            94 => "Plutonium",
            95 => "Americium",
            96 => "Curium",
            97 => "Berkelium",
            98 => "Californium",
            99 => "Einsteinium",
            100 => "Fermium",
            101 => "Mendelevium",
            102 => "Nobelium",
            103 => "Lawrencium",
            104 => "Rutherfordium",
            105 => "Dubnium",
            106 => "Seaborgium",
            107 => "Bohrium",
            108 => "Hassium",
            109 => "Meitnerium",
            110 => "Darmstadtium",
            111 => "Roentgenium",
            112 => "Copernicium",
            113 => "Nihonium",
            114 => "Flerovium",
            115 => "Moscovium",
            116 => "Livermorium",
            117 => "Tennessine",
            118 => "Oganesson",
            _ => "Unknown",
        }
    }
}

fn update_atom_label_text(
    mut query: Query<(&mut Text, &mut AtomLabelText)>,
    query_electrons: Query<&Electron>,
) {
    let count = query_electrons.iter().count();

    for (mut text, mut label) in &mut query {
        if count == label.0 {
            return;
        }
        label.0 = count;
        text.sections[0].value = format!("{}", label.label());
    }
}
