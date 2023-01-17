use nanoid::nanoid;
use crate::types::Text;
use serde::{Deserialize,Serialize};
use std::prelude::*;

#[derive(Debug,  Eq, PartialEq, Clone)]
pub struct Uid {
	value: Text
}

impl Uid {
	pub fn new() -> Self {
		Uid {value: nanoid!() }
	}
}

