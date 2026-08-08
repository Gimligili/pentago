from pentago_ai import pentago_engine
from pentago_ai.agent import NeuralAgent
from pentago_ai.model import PentagoNet


def test_agent_returns_legal_action():
    game = pentago_engine.PyGame()

    model = PentagoNet()
    agent = NeuralAgent(model)

    action = agent.choose_action(game)

    assert action in game.legal_actions()


def test_agent_returns_legal_action_after_move():
    game = pentago_engine.PyGame()

    first_action = game.legal_actions()[0]
    game.step(first_action)

    model = PentagoNet()
    agent = NeuralAgent(model)

    action = agent.choose_action(game)

    assert action in game.legal_actions()