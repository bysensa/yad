
/// Should contain MimeType, Encoding and encoded data
pub struct Binary;

/// The boolean type
pub type Boolean = bool;

pub type Uri = uriparse::URI<'static>;

pub type CalAddress = Uri;

pub type Date = chrono::NaiveDate;

pub type DateTime = chrono::DateTime<chrono::Utc>;

pub type Duration = chrono::Duration;

pub type Time = chrono::NaiveTime;

pub type Float = f64;

pub type Integer = i64;

pub struct Period(DateTime, DateTime);

pub type Recurrence = rrule::RRuleSet;

pub type Text = String;

pub type UtcOffset = chrono::Duration;

pub type Id = String;

pub enum Value {
	Binary(Binary),
	Boolean(Boolean),
	Uri(Uri),
	CalAddress(CalAddress),
	Date(Date),
	DateTime(DateTime),
	Duration(Duration),
	Time(Time),
	Float(Float),
	Integer(Integer),
	Period(Period),
	Recurrence(Recurrence),
	Text(Text),
	UtcOffset(UtcOffset),
}