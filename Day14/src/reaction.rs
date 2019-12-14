use snafu::{ensure, ResultExt, Snafu};
use std::hash::Hash;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Chemical {
    Ore,
    Fuel,
    Intermediary(String),
}

#[derive(Debug)]
pub struct ChemicalQuantity {
    chemical: Chemical,
    coefficient: u8,
}

#[derive(Debug)]
pub struct Reaction {
    inputs: Vec<ChemicalQuantity>,
    output: ChemicalQuantity,
}

#[derive(Debug, Snafu)]
pub enum ParseChemicalError {
    #[snafu(display("Invalid Chemical: empty string"))]
    EmptyString,
}

impl FromStr for Chemical {
    type Err = ParseChemicalError;
    fn from_str(s: &str) -> Result<Chemical, Self::Err> {
        match s.trim() {
            "" => Err(ParseChemicalError::EmptyString),
            "ORE" => Ok(Chemical::Ore),
            "FUEL" => Ok(Chemical::Fuel),
            _ => {
                // let mut hasher = DefaultHasher::new();
                // s.hash(&mut hasher);
                // Ok(Chemical::Intermediary(hasher.finish()))
                Ok(Chemical::Intermediary(s.to_owned()))
            }
        }
    }
}

#[derive(Debug, Snafu)]
pub enum ParseChemicalQuantityError {
    #[snafu(display("Invalid chemical: {}", source))]
    InvalidChemical { source: ParseChemicalError },
    #[snafu(display("Invalid chemical coefficient \"{}\": {}", coeff, source))]
    InvalidCoefficient {
        coeff: String,
        source: ParseIntError,
    },
    #[snafu(display("Invalid Chemical quantity: {}", quantity))]
    InvalidChemicalQuantity { quantity: String },
}

impl FromStr for ChemicalQuantity {
    type Err = ParseChemicalQuantityError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split_whitespace().collect::<Vec<_>>();
        ensure!(
            parts.len() == 2,
            InvalidChemicalQuantity {
                quantity: s.to_owned(),
            }
        );
        let coefficient = parts[0].parse::<u8>().context(InvalidCoefficient {
            coeff: parts[0].to_owned(),
        })?;
        let chemical = parts[1].parse::<Chemical>().context(InvalidChemical {})?;
        Ok(ChemicalQuantity {
            chemical,
            coefficient,
        })
    }
}

#[derive(Debug, Snafu)]
pub enum ParseReactionError {
    #[snafu(display("Invalid reaction syntax: {}", reaction))]
    InvalidReactionSyntax { reaction: String },
    //#[snafu(display("The reaction must produce one type of chemical, not {}: {}", number, outputs))]
    //InvalidNumberOfOuputs { outputs: String, number: usize},
    #[snafu(display("The reaction must take at least one input: {}", reaction))]
    NoInputs { reaction: String },
    #[snafu(display("Invalid input chemical: {}", source))]
    InvalidInputChemical { source: ParseChemicalQuantityError },
    #[snafu(display("Invalid onput chemical: {}", source))]
    InvalidOutputChemical { source: ParseChemicalQuantityError },
}

impl FromStr for Reaction {
    type Err = ParseReactionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("=>").collect::<Vec<_>>();
        ensure!(
            parts.len() == 2,
            InvalidReactionSyntax {
                reaction: s.to_owned()
            }
        );

        let inputs = parts[0].split(',').collect::<Vec<_>>();
        ensure!(
            inputs.len() >= 1,
            NoInputs {
                reaction: s.to_owned()
            }
        );

        let inputs = inputs
            .iter()
            .map(|s| s.parse::<ChemicalQuantity>())
            .collect::<Result<Vec<_>, _>>()
            .context(InvalidInputChemical {})?;
        let output = parts[1]
            .parse::<ChemicalQuantity>()
            .context(InvalidInputChemical {})?;
        Ok(Reaction { inputs, output })
    }
}

impl Reaction {
    pub fn output_chemical(&self) -> &Chemical {
        &self.output.chemical
    }

    pub fn inputs(&self) -> &Vec<ChemicalQuantity> {
        &self.inputs
    }

    pub fn output(&self) -> &ChemicalQuantity {
        &self.output
    }
}

impl ChemicalQuantity {
    pub fn chemical(&self) -> &Chemical {
        &self.chemical
    }

    pub fn coefficient(&self) -> u8 {
        self.coefficient
    }
}
