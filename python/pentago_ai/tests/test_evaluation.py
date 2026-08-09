from pentago_ai.agent import NeuralAgent
from pentago_ai.evaluation import evaluate_agent, play_game
from pentago_ai.model import PentagoNet


def test_play_game_reaches_terminal_state():
    model = PentagoNet()
    agent = NeuralAgent(
        model,
        stochastic=False,
    )

    result, _, _, _ = play_game(
        neural_agent=agent,
        neural_player=1,
    )

    assert result in (1, -1, 2)

def test_evaluate_agent_counts_all_games():
    model = PentagoNet()

    agent = NeuralAgent(
        model,
        stochastic=False,
    )

    result = evaluate_agent(
        neural_agent=agent,
        num_games=4,
    )

    assert result.total_games == 4
    assert result.wins >= 0
    assert result.losses >= 0
    assert result.draws >= 0

    assert (
        result.wins
        + result.losses
        + result.draws
        == 4
    )

def test_play_game_with_random_opening_reaches_terminal_state():
    model = PentagoNet()

    agent = NeuralAgent(
        model,
        stochastic=False,
    )

    result, compared, matching, _ = play_game(
    neural_agent=agent,
    neural_player=1,
    )

    assert result in (1, -1, 2)
    assert compared >= 0
    assert matching >= 0
    assert matching <= compared
