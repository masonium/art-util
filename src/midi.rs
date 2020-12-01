//! Translate raw midi messages into strongly typed ones.
pub use std::convert::TryFrom;

pub enum MessageError {
    UnknownMessage(u8),
    BadLength(usize)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Message {
    pub channel: u8,
    pub data: MessageData
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MessageData {
    NoteOn { node: u8, velocity: u8 },
    NoteOff { node: u8, velocity: u8 },
    ControlChange { control: u8, value: u8 },
    ChannelMode { control: u8, value: u8 }
}


impl TryFrom<&[u8]> for Message {
    type Error = MessageError;
    fn try_from(bytes: &[u8]) -> Result<Message, Self::Error> {
	let n = bytes.len();
	if n == 0 || n > 3 {
	    return Err(MessageError::BadLength(n));
	}

	let channel = bytes[0] & 0x0f;
	let msg_type =  bytes[0] >> 4;

	let data = if n == 3 {
	    match msg_type {
		0x8 => MessageData::NoteOff { node: bytes[1], velocity: bytes[2] },
		0x9 => MessageData::NoteOn { node: bytes[1], velocity: bytes[2] },
		0xb => {
		    if bytes[1] < 120 {
			MessageData::ControlChange { control: bytes[1], value: bytes[2] }
		    } else {
			MessageData::ChannelMode { control: bytes[1], value: bytes[2] }
		    }
		},
		_ => return Err(MessageError::UnknownMessage(msg_type))
	    }
	} else {
	    return Err(MessageError::BadLength(n));
	};

	Ok(Message { channel, data })
    }
}
