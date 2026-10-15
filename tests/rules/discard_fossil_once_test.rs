//! A Fossil in play can be discarded during your turn: one `DiscardFossil` for each Fossil, not two.

use deckgym::{
    actions::SimpleAction, card_ids::CardId, models::PlayedCard,
    move_generation::generate_possible_actions, test_support::get_test_game_with_board,
};

#[test]
fn each_fossil_in_play_is_offered_to_discard_once() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1216HelixFossil),
            PlayedCard::from_id(CardId::A2144SkullFossil),
        ],
        vec![PlayedCard::from_id(CardId::A1005Caterpie)],
    );
    let (_, actions) = generate_possible_actions(&game.get_state_clone());
    let discards: Vec<SimpleAction> = actions
        .into_iter()
        .map(|action| action.action)
        .filter(|action| matches!(action, SimpleAction::DiscardFossil { .. }))
        .collect();
    assert_eq!(
        discards,
        vec![
            SimpleAction::DiscardFossil { in_play_idx: 1 },
            SimpleAction::DiscardFossil { in_play_idx: 2 },
        ]
    );
}
