mod payout_destination_register;
mod payout_destination_remove;

pub use payout_destination_register::{
    PayoutDestinationRegisterCommand, PayoutDestinationRegisterCommandHandler,
    PayoutDestinationRegisterCommandHandlerError, PayoutDestinationRegisterOutput,
};
pub use payout_destination_remove::{
    PayoutDestinationRemoveCommand, PayoutDestinationRemoveCommandHandler,
    PayoutDestinationRemoveCommandHandlerError, PayoutDestinationRemoveOutput,
};
