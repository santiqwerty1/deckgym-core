//! With no Active Pokémon (after a Knock Out, before the promotion, as search players explore),
//! move generation must not panic: the abilities that need the Active are simply not usable.

use deckgym::{
    card_ids::CardId, models::PlayedCard, move_generation::generate_possible_actions,
    test_support::get_test_game_with_board,
};

/// Player 0 has no Active and `bench` on its Bench (from the second spot on).
fn no_active(bench: Vec<CardId>) -> deckgym::State {
    let mut board = vec![PlayedCard::from_id(CardId::A1001Bulbasaur)];
    board.extend(bench.into_iter().map(PlayedCard::from_id));
    let game = get_test_game_with_board(board, vec![PlayedCard::from_id(CardId::A1005Caterpie)]);
    let mut state = game.get_state_clone();
    state.in_play_pokemon[0][0] = None;
    state
}

#[test]
fn ice_maker_with_no_active_pokemon_does_not_panic() {
    // Baxcalibur's Ice Maker (attach a [W] Energy from the zone to the Active) checks the Active.
    let state = no_active(vec![CardId::B2a034Frigibax, CardId::B2a036Baxcalibur]);
    let _ = generate_possible_actions(&state);
}

#[test]
fn every_ability_that_reads_the_active_tolerates_none() {
    // The other abilities that read the Active in move generation: Celesteela's Ultra Thrusters,
    // the switch-with-Bench abilities of a type and Vaporeon's Wash Out.
    for bench in [
        vec![CardId::A3a062Celesteela],
        vec![CardId::A1a019Vaporeon],
        vec![CardId::B2a036Baxcalibur, CardId::A1a019Vaporeon],
    ] {
        let state = no_active(bench);
        let _ = generate_possible_actions(&state);
    }
}
