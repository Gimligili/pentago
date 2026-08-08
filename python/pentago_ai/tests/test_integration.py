from pentago_ai import pentago_engine
from pentago_ai.agent import NeuralAgent
from pentago_ai.encoding import encode_board
from pentago_ai.model import ACTION_COUNT, PentagoNet


def test_model_accepts_encoded_game():
    game = pentago_engine.PyGame()

    state = encode_board(game)

    model = PentagoNet()

    policy, value = model(state.unsqueeze(0))

    assert policy.shape == (1, ACTION_COUNT)
    assert value.shape == (1, 1)

def test_neural_agents_can_finish_game():
    game = pentago_engine.PyGame()

    model = PentagoNet()
    agent = NeuralAgent(model)

    max_moves = 36

    for _ in range(max_moves):
        if game.game_status() != 0:
            break

        action = agent.choose_action(game)
        game.step(action)

    assert game.game_status() != 0