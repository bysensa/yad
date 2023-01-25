use entrait::entrait;

use crate::model::{ActivatedApplication, FocusTime, FocusSwitch};



#[entrait(pub TrackApplicatiuonChange, box_future)]
pub async fn track_application_change(_deps: &impl std::any::Any, event: ActiveApplicationChange) {
    let focus_time = event.focus_time();
    let focus_switch = event.focus_switch();
}

#[derive(Debug)]
pub struct ActiveApplicationChange {
    current: ActivatedApplication,
    previous: ActivatedApplication
}

impl ActiveApplicationChange {
    fn focus_time(&self) -> FocusTime {
        let duration = self.current.at.signed_duration_since(self.previous.at);
        let duration = std::time::Duration::from_millis(duration.num_milliseconds().unsigned_abs());
        FocusTime {
            on: self.previous.application.clone(),
            during: duration,
            since: self.previous.at.clone(),
        }
    }

    fn focus_switch(&self) -> FocusSwitch {
        FocusSwitch { from: self.previous.application.clone(), to: self.current.application.clone(), at: self.current.at.clone() }
    }
}