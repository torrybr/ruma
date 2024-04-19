//! `Serialize` and `Deserialize` implementations for extensible events (MSC1767).

use serde::Deserialize;

use super::{BeaconAnswer, BeaconAnswers, BeaconAnswersError};

#[derive(Debug, Default, Deserialize)]
pub(crate) struct BeaconAnswersDeHelper(Vec<BeaconAnswer>);

impl TryFrom<BeaconAnswersDeHelper> for BeaconAnswers {
    type Error = BeaconAnswersError;

    fn try_from(helper: BeaconAnswersDeHelper) -> Result<Self, Self::Error> {
        let mut answers = helper.0;
        answers.truncate(BeaconAnswers::MAX_LENGTH);
        BeaconAnswers::try_from(answers)
    }
}
