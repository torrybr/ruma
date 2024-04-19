//! `Deserialize` helpers for unstable beacon answers (MSC3381).

use serde::Deserialize;

use super::{UnstableBeaconAnswer, UnstableBeaconAnswers};
use crate::beacon::start::{BeaconAnswers, BeaconAnswersError};

#[derive(Debug, Default, Deserialize)]
pub(crate) struct UnstableBeaconAnswersDeHelper(Vec<UnstableBeaconAnswer>);

impl TryFrom<UnstableBeaconAnswersDeHelper> for UnstableBeaconAnswers {
    type Error = BeaconAnswersError;

    fn try_from(helper: UnstableBeaconAnswersDeHelper) -> Result<Self, Self::Error> {
        let mut answers = helper.0;
        answers.truncate(BeaconAnswers::MAX_LENGTH);
        UnstableBeaconAnswers::try_from(answers)
    }
}
