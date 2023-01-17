use ics::components::Parameters;
use crate::types::*;
use crate::parameters::*;


enum Attachment {
	Uri(Uri),
	Binary(Binary)
}

struct Category(Text);

struct Categories(Vec<Category>);

enum Classification {
	Public,
	Private,
	Confidential,
	Iana(Text),
	Custom(Text)
}

struct Comment {
	value: Text,
	lang: Option<Language>,
	altrep: Option<AlternativeRepresentation>,
	parameters: Vec<Parameter>,
}

pub struct Description {
	value: Text,
	lang: Option<Language>,
	altrep: Option<AlternativeRepresentation>,
	parameters: Vec<Parameter>,
}

pub struct GeoPosition {
	value: geo::Point,
	parameters: Vec<Parameter>,
}

pub struct Location {
	value: Text,
	lang: Option<Language>,
	altrep: Option<AlternativeRepresentation>,
	parameters: Vec<Parameter>,
}

pub struct CompletionPercent {
	value: Integer,
	parameters: Vec<Parameter>,
}

pub struct Priority {
	value: Integer,
	parameters: Vec<Parameter>,
}

pub struct Resources {
	value: Text,
	lang: Option<Language>,
	altrep: Option<AlternativeRepresentation>,
	parameters: Vec<Parameter>,
}

pub struct Status {
	value: StatusValue,
	parameters: Vec<Parameter>,
}

pub enum StatusValue {
	Tentative,
	Confirmed,
	NeedsAction,
	Completed,
	InProcess,
	Draft,
	Final,
	Cancelled,
}