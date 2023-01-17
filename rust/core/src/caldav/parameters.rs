use crate::types::*;


pub enum Parameter {
	Altrep(AlternativeRepresentation),
	CommonName(CommonName),
	UserType(UserType),
	Delegators(Delegators),
	Delegatees(Delegatees),
	Directory(Directory),
	Encoding(Encoding),
	FormatType(FormatType),
	FreeBusy(FreeBusy),
	Language(Language),
	Members(Members),
	ParticipationStatus(ParticipationStatus),
	AlarmTriggerRelationship(AlarmTriggerRelationship),
	RelationshipType(RelationshipType),
	ParticipationRole(ParticipationRole),
	Rsvp(Rsvp),
	SentBy(SentBy),
	TzId(TzId),
	ValueType(ValueType),
	Iana(Text, Vec<Value>),
	Custom(Text, Vec<Value>)
}

pub struct Language(language_tags::LanguageTag);

pub struct AlternativeRepresentation(Uri);

pub struct CommonName(Text);

pub struct Delegators(Vec<Delegator>);

pub struct Delegator(CalAddress);

pub struct Delegatees(Vec<Delegatee>);

pub struct Delegatee(CalAddress);

pub struct Directory(Uri);

pub struct FormatType(Text);

pub struct Members(Vec<Member>);

pub struct Member(CalAddress);

pub struct Rsvp(Boolean);

pub enum UserType {
	Individual,
	Group,
	Resource,
	Room,
	Unknown,
	Iana(Text),
	Custom(Text),
}

pub enum Encoding {
	Bit8,
	Base64,
}

pub enum FreeBusy {
	Free,
	Busy,
	Unavailable,
	Tentative,
	Iana(Text),
	Custom(Text)
}

pub enum ParticipationStatus {
	NeedsAction,
	Accepted,
	Declined,
	Tentative,
	Delegated,
	Completed,
	InProcess,
	Custom(Text),
	Iana(Text),
}

pub enum AlarmTriggerRelationship {
	Start,
	End,
}

pub enum RelationshipType {
	Parent,
	Child,
	Sibling,
	Iana(Text),
	Custom(Text),
}

pub enum ParticipationRole {
	Chair,
	RequiredParticipant,
	OptionalParticipant,
	NonParticipant,
}

pub struct SentBy(CalAddress);

pub struct TzId(Text);

pub enum ValueType {
	Boolean,
	Float,
	Integer,
	Text,
	Uri,
	CalAddress,
	Date,
	DateTime,
	Duration,
	Time,
	Period,
	Binary,
	Recurrence,
	UtcOffset,
}