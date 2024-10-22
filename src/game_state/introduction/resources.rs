use bevy::prelude::*;

// The global resource that holds the game state
#[derive(Resource, Default)]
pub struct IntroductionData {
    pub step: IntroductionStep,
}

// The different steps of the introduction
#[derive(Debug, PartialEq, Default)]
pub enum IntroductionStep {
    #[default]
    StepOne,
    StepTwo,
    End,
}

impl IntroductionStep {
    pub fn next(&self) -> Self {
        match self {
            Self::StepOne => Self::StepTwo,
            Self::StepTwo => Self::End,
            Self::End => Self::End,
        }
    }
}
