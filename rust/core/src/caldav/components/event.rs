use chrono::Utc;
use crate::properties::Uid;
use crate::types::*;

#[derive(Debug, Clone)]
pub struct Event {
	uid: Uid,
	// In the case of an iCalendar object that doesn't specify a "METHOD" property,
	// this property is equivalent to the "LAST-MODIFIED" property.
	dt_stamp: DateTime,
	// This property specifies the date and time that the calendar information was
	// created by the calendar user agent in the calendar store.
	dt_created: DateTime,
	// This property specifies the date and time that the information associated
	// with the calendar component was last revised in the calendar store.
	dt_modified: DateTime,
	// When a calendar component is created, its sequence number is 0.
	// It is monotonically incremented by the "Organizer's" CUA each time the
	// "Organizer" makes a significant revision to the calendar component.
	seq_number: Integer,
	// this property defines the start date and time for the event.
	dt_start: DateTime,
	// the property used to specify a duration of the event, instead of an
	// explicit end DATE-TIME
	duration: Duration,
	//
	// classification: Option<>,
	// description: Option<?>,
	// geo: Option<?>,
	// location: Option<?>,
	// organizer: Option<?>,
	// priority: Option<?>,
	// status: Option<?>,
	// summary: Option<?>,
	// transparency: Option<?>,
	// url: Option<?>,
	// recurrence_id: Option<?>,
	// recurrence_rule: Option<?>,
	// //
	// attach: Option<Vec<?>>,
	// attendee: Option<Vec<?>>,
	// categories: Option<Vec<?>>,
	// comment: Option<Vec<?>>,
	// contact: Option<Vec<?>>,
	// exception_dt: Option<Vec<?>>,
	// request_status: Option<Vec<?>>,
	// related: Option<Vec<?>>,
	// resources: Option<Vec<?>>,
	// recurrence_dt: Option<Vec<?>>,
}

impl Event {
	fn create() -> Self {
		let now = Utc::now();
		Event {
			uid: Uid::new(),
			dt_stamp: now.clone(),
			dt_created: now.clone(),
			dt_modified: now.clone(),
			dt_start: now.clone(),
			duration: Duration::zero(),
			seq_number: 0,
			// classification: None,
			// description: None,
			// geo: None,
			// location: None,
			// organizer: None,
			// priority: None,
			// status: None,
			// summary: None,
			// transparency: None,
			// url: None,
			// recurrence_id: None,
			// recurrence_rule: None,
			// attach: None,
			// attendee: None,
			// categories: None,
			// comment: None,
			// contact: None,
			// exception_dt: None,
			// request_status: None,
			// related: None,
			// resources: None,
			// recurrence_dt: None
		}
	}
}