use crate::data::locations::Demographic::{Border, Rural, Urban};
use std::fmt::{Display, Formatter};

// todo there's not a 1-1 correspondence from culture to language - should those be explicit?
// the document seems to suggest there should be... ask lys
#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Culture {
    Varlish,
    Kremish,
    Revic,
    Myrsc(Faith),
    Clovienne,
    Grevolin,
    Shander,
    Torienne,
}

impl Display for Culture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Culture::Myrsc(_) => write!(f, "Myrsc"),
            other => write!(f, "{:?}", other),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Language {
    // todo
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Faith {
    Accorder,
    IdealLodges,
    Gytungrug,
    OrodTast,
    Blaithsworn,
    KeystonePantheon,
    Grevite,
    Flametouched,
    AmberheartCult,
    UnseenHand,
    TempleOfSeraf,
    Irreligious,
}

impl Display for Faith {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Faith::Accorder => "White Flame Accord",
            Faith::IdealLodges => "Ideal Lodges",
            Faith::Gytungrug => "Gytungrug",
            Faith::OrodTast => "Orod Tâst",
            Faith::Blaithsworn => "Blaithsworn",
            Faith::KeystonePantheon => "Keystone Pantheon",
            Faith::Grevite => "Grevite",
            Faith::Flametouched => "Flametouched",
            Faith::AmberheartCult => "Amberheart Cult",
            Faith::UnseenHand => "The Unseen Hand",
            Faith::TempleOfSeraf => "The Temple of Seraf",
            Faith::Irreligious => "Irreligious",
        };
        write!(f, "{}", name)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Demographic {
    Urban,
    Rural,
    Border,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum CareerTable {
    ValiantEmpire,
    Nivena,
    Marolaw,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Location {
    pub name: String,
    pub culture: Culture,
    pub secondary_culture: Culture,
    pub faith: Faith,
    pub secondary_faith: Faith,
    pub demographic: Demographic,
    pub career_table: CareerTable,
    pub far_afield: bool,
}

pub fn location_table(d6s: (i8, i8, i8), d3: i8) -> Location {
    let culture;
    let faith;
    let secondary_faith;
    let name;
    let secondary_culture;
    let demographic;
    let career_table;
    let far_afield;

    match d6s.0 {
        1 | 2 => {
            // Valiant empire
            career_table = CareerTable::ValiantEmpire;
            culture = Culture::Varlish;
            faith = Faith::Accorder;
            secondary_faith = Faith::IdealLodges;
            far_afield = false;
            match d6s.1 {
                1 => {
                    name = "The Imperial Capital of Duruza";
                    secondary_culture = Culture::Kremish;
                    demographic = Demographic::Urban;
                }
                2 => {
                    name = "Luruna, the North Sentinel";
                    secondary_culture = Culture::Kremish;
                    demographic = Demographic::Urban;
                }
                3 => {
                    name = "The volcanic farmlands around Caldis";
                    secondary_culture = Culture::Kremish;
                    demographic = Demographic::Rural;
                }
                4 => {
                    name = "The treacherous Mervaal marshes";
                    secondary_culture = Culture::Kremish;
                    demographic = Demographic::Rural;
                }
                5 => {
                    name = "Along the dusty Bone Road to Orvaal";
                    secondary_culture = Culture::Revic;
                    demographic = Demographic::Border;
                }
                6 => {
                    name = "In the shadow of Mount Cothornis";
                    secondary_culture = Culture::Revic;
                    demographic = Demographic::Border;
                }
                _ => unreachable!(),
            }
        }
        3 => {
            // Nivena
            career_table = CareerTable::Nivena;
            culture = Culture::Varlish;
            faith = Faith::IdealLodges;
            far_afield = false;
            match d6s.1 {
                1 => {
                    name = "Niiva, City of Intrigues";
                    secondary_culture = Culture::Kremish;
                    secondary_faith = Faith::Gytungrug;
                    demographic = Demographic::Urban;
                }
                2 => {
                    name = "The Fortified Port of Murga";
                    secondary_culture = Culture::Revic;
                    secondary_faith = Faith::OrodTast;
                    demographic = Demographic::Urban;
                }
                3 => {
                    name = "The riverlands around Faraxi";
                    secondary_culture = Culture::Kremish;
                    secondary_faith = Faith::Gytungrug;
                    demographic = Demographic::Rural;
                }
                4 => {
                    name = "Around the Chalys Pinewoods";
                    secondary_culture = Culture::Kremish;
                    secondary_faith = Faith::Gytungrug;
                    demographic = Demographic::Rural;
                }
                5 => {
                    name = "In the barren, hilly Sorrow";
                    secondary_culture = Culture::Kremish;
                    secondary_faith = Faith::Accorder;
                    demographic = Demographic::Border;
                }
                6 => {
                    name = "A colony on an island or by Suruso";
                    secondary_culture = Culture::Revic;
                    secondary_faith = Faith::OrodTast;
                    demographic = Demographic::Border;
                }
                _ => unreachable!(),
            }
        }
        4 | 5 => {
            // Marolaw
            career_table = CareerTable::Marolaw;
            culture = Culture::Revic;
            secondary_culture = Culture::Varlish;
            faith = Faith::Accorder;
            far_afield = false;
            match d6s.1 {
                1 => {
                    name = "The Administrative Capital at Marodell";
                    secondary_faith = Faith::OrodTast;
                    demographic = Demographic::Urban;
                }
                2 => {
                    name = "Varnoss, the Bloody Bulwark";
                    secondary_faith = Faith::OrodTast;
                    demographic = Demographic::Urban;
                }
                3 => {
                    name = "Stravarn and the lush southeast coast";
                    secondary_faith = Faith::OrodTast;
                    demographic = Demographic::Rural;
                }
                4 => {
                    name = "The rugged woodland of the Harsten Vale";
                    secondary_faith = Faith::OrodTast;
                    demographic = Demographic::Rural;
                }
                5 => {
                    name = "The sun-bleached Dry Coast";
                    secondary_faith = Faith::IdealLodges;
                    demographic = Demographic::Border;
                }
                6 => {
                    name = "The wood and marshland of the Rimelight";
                    secondary_faith = Faith::IdealLodges;
                    demographic = Demographic::Border;
                }
                _ => unreachable!(),
            }
        }
        6 => {
            // Somewhere further afield
            far_afield = true;
            match d6s.1 {
                1 => {
                    name = "The Fallen Kingdom of  Kr’meche";
                    culture = Culture::Kremish;
                    faith = Faith::Gytungrug;
                    career_table = CareerTable::ValiantEmpire;
                }
                2 => {
                    name = "Gollg’rym, the frost-bitten wastes";
                    culture = Culture::Kremish;
                    faith = Faith::Gytungrug;
                    career_table = CareerTable::Marolaw;
                }
                3 => {
                    name = "The Ash Mountains, or the Sea of Shades";
                    culture = Culture::Revic;
                    faith = Faith::OrodTast;
                    career_table = CareerTable::Marolaw;
                }
                4 => {
                    name = "The isles of the Carmine Sea Compact";
                    culture = Culture::Myrsc(Faith::KeystonePantheon);
                    faith = Faith::KeystonePantheon;
                    career_table = CareerTable::Nivena;
                }
                5 => {
                    name = "The bustling Thousand Spires Coast";
                    culture = Culture::Clovienne;
                    faith = Faith::Accorder;
                    career_table = CareerTable::ValiantEmpire;
                }
                6 => {
                    // Even Further
                    match d6s.2 {
                        1 => {
                            name = "The hidebound, riven Rose Coast";
                            culture = Culture::Grevolin;
                            faith = Faith::Grevite;
                            career_table = CareerTable::Nivena;
                        }
                        2 => {
                            name = "The plague-blighted Crown Coast";
                            culture = Culture::Grevolin;
                            faith = Faith::Flametouched;
                            career_table = CareerTable::Nivena;
                        }
                        3 => {
                            name = "A city clinging to the Umbral Coast";
                            culture = Culture::Myrsc(Faith::AmberheartCult);
                            faith = Faith::AmberheartCult;
                            career_table = CareerTable::Nivena;
                        }
                        4 => {
                            name = "The blasted dunes of the Great Flat";
                            culture = Culture::Shander;
                            faith = Faith::Accorder;
                            career_table = CareerTable::Marolaw;
                        }
                        5 => {
                            name = "Neminyah, the rotting heart";
                            culture = Culture::Myrsc(Faith::UnseenHand);
                            faith = Faith::UnseenHand;
                            career_table = CareerTable::Marolaw;
                        }
                        6 => {
                            name = "The brackish jungles of Rezankath";
                            culture = Culture::Torienne;
                            faith = Faith::TempleOfSeraf;
                            career_table = CareerTable::ValiantEmpire;
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
            demographic = match d3 {
                1 => Urban,
                2 => Rural,
                3 => Border,
                _ => unreachable!(),
            };
            // these locations never use the secondary faith/culture mechanic and don't list any
            secondary_faith = faith;
            secondary_culture = culture;
        }
        _ => unreachable!(),
    }
    Location {
        name: name.to_owned(),
        culture,
        secondary_culture,
        faith,
        secondary_faith,
        demographic,
        career_table,
        far_afield,
    }
}

// pulled out due to being used in multiple places
pub fn further_afield_culture(d6s: (i8, i8)) -> Culture {
    match d6s.0 {
        1 => Culture::Kremish,
        2 => Culture::Kremish,
        3 => Culture::Revic,
        4 => Culture::Myrsc(Faith::KeystonePantheon),
        5 => Culture::Clovienne,
        6 => {
            // Even Further
            match d6s.1 {
                1 => Culture::Grevolin,
                2 => Culture::Grevolin,
                3 => Culture::Myrsc(Faith::AmberheartCult),
                4 => Culture::Shander,
                5 => Culture::Myrsc(Faith::AmberheartCult),
                6 => Culture::Torienne,
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

pub fn associated_faith(culture: Culture) -> Faith {
    match culture {
        Culture::Varlish => Faith::Accorder,
        Culture::Kremish => Faith::Gytungrug,
        Culture::Revic => Faith::OrodTast,
        Culture::Myrsc(faith) => faith,
        Culture::Clovienne => Faith::Accorder,
        Culture::Grevolin => Faith::Grevite,
        Culture::Shander => Faith::Accorder,
        Culture::Torienne => Faith::TempleOfSeraf,
    }
}

pub fn get_culture(location: &Location, d6s: (i8, i8, i8, i8)) -> Culture {
    if location.far_afield {
        return location.culture;
    }

    match d6s.0 {
        1 | 2 | 3 => location.culture,
        4 | 5 => location.secondary_culture,
        6 => match d6s.1 {
            1 | 2 | 3 => Culture::Varlish,
            4 => Culture::Revic,
            5 => Culture::Kremish,
            6 => further_afield_culture((d6s.2, d6s.3)),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

pub fn get_faith(location: &Location, culture: Culture, d6s: (i8, i8)) -> Faith {
    if location.far_afield {
        return location.faith;
    }

    match d6s.0 {
        1 | 2 => {
            if culture == Culture::Kremish {
                Faith::Gytungrug
            } else {
                location.faith
            }
        }
        3 => location.secondary_faith,
        4 | 5 => associated_faith(culture),
        6 => match d6s.1 {
            1 | 2 | 3 => Faith::Accorder,
            4 | 5 => Faith::Irreligious,
            6 => Faith::TempleOfSeraf,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
