//! What bots read from the engine without playing a move: the expected damage of an attack, the
//! forecast of an action and the retreat cost with every effect applied.

use deckgym::{
    actions::{expected_attack_damage, forecast_action, Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    get_retreat_cost,
    models::{Card, PlayedCard},
    test_support::get_test_game_with_board,
};

fn attack_of(card_id: CardId) -> deckgym::models::Attack {
    match get_card_by_enum(card_id) {
        Card::Pokemon(pokemon) => pokemon.attacks[0].clone(),
        _ => panic!("a Pokémon"),
    }
}

#[test]
fn expected_attack_damage_counts_the_tools_dede_circuit_needs() {
    // Dedenne ex's Dede-Circuit: 40 damage for each of your Pokémon that has a Pokémon Tool.
    let mut dedenne = PlayedCard::from_id(CardId::B3b024DedenneEx);
    let game = get_test_game_with_board(vec![dedenne.clone()], vec![PlayedCard::from_id(CardId::A1001Bulbasaur)]);
    let circuit = attack_of(CardId::B3b024DedenneEx);
    assert_eq!(expected_attack_damage(&game.get_state_clone(), 0, &circuit, 1, 0), 0.0);
    dedenne.attached_tools.push(get_card_by_enum(CardId::A2147GiantCape));
    let game = get_test_game_with_board(vec![dedenne], vec![PlayedCard::from_id(CardId::A1001Bulbasaur)]);
    assert_eq!(expected_attack_damage(&game.get_state_clone(), 0, &circuit, 1, 0), 40.0);
}

#[test]
fn forecast_action_and_get_retreat_cost_are_public() {
    let game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let state = game.get_state_clone();
    let end_turn = Action { actor: 0, action: SimpleAction::EndTurn, is_stack: false };
    let _ = forecast_action(&state, &end_turn);
    let bulbasaur = state.in_play_pokemon[0][0].as_ref().expect("Bulbasaur");
    assert_eq!(get_retreat_cost(&state, bulbasaur).len(), 1);
}
