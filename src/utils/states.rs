/// Represents the login state of a user.
///
/// # Variants
/// - `NotLoggedIn`: The user is not logged in.
/// - `LoggedIn`: The user is logged in.
#[derive(Clone, PartialEq)]
pub enum LoginState {
    NotLoggedIn,
    LoggedIn,
}

/// Represents the state of whether a card is new.
///
/// # Fields
/// - `0`: A boolean indicating if the card is new.
#[derive(Clone, Copy)]
pub struct IsNewCardState(pub bool);

/// Represents the state of whether a selection is being made.
///
/// # Fields
/// - `0`: A boolean indicating if a selection is being made.
#[derive(Clone, Copy)]
pub struct IsSelectingState(pub bool);
