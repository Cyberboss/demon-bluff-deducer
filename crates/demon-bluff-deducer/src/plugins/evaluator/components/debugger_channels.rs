use async_channel::{Receiver, Sender, TryRecvError};
use bevy::{ecs::component::Component, tasks::block_on};
use demon_bluff_logic_engine::Breakpoint;

#[derive(Component)]
pub struct DebuggerChannels {
	continue_sender: Sender<()>,
	breakpoint_receiver: Receiver<Breakpoint>,
}

impl DebuggerChannels {
	pub fn new() -> (Self, Receiver<()>, Sender<Breakpoint>) {
		let (continue_sender, continue_receiver) = async_channel::unbounded();
		let (breakpoint_sender, breakpoint_receiver) = async_channel::unbounded();

		(
			Self {
				continue_sender,
				breakpoint_receiver,
			},
			continue_receiver,
			breakpoint_sender,
		)
	}

	pub fn try_get_breakpoint(&self) -> Result<Option<Breakpoint>, ()> {
		match self.breakpoint_receiver.try_recv() {
			Ok(breakpoint) => Ok(Some(breakpoint)),
			Err(err) => match err {
				TryRecvError::Empty => Ok(None),
				TryRecvError::Closed => Err(()),
			},
		}
	}

	pub fn send_continue(&self) {
		if let Err(_) = block_on(self.continue_sender.send(())) {
			panic!("Continue channel was closed unexpectedly")
		}
	}
}
