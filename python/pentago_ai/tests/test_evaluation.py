from pentago_ai.agent import NeuralAgent
from pentago_ai.evaluation import evaluate_agent, play_game
from pentago_ai.model import PentagoNet


def test_play_game_reaches_terminal_state():
    model = PentagoNet()
    agent = NeuralAgent(
        model,
        stochastic=False,
    )

    result = play_game(
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

def test_evaluation_result_win_rate():
    from pentago_ai.evaluation import EvaluationResult

    result = EvaluationResult(
        wins=2,
        losses=1,
        draws=1,
    )

    assert result.total_games == 4
    assert result.win_rate == 0.5
