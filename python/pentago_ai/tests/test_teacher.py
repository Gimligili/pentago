from pentago_ai import pentago_engine


def test_teacher_returns_legal_action():
    game = pentago_engine.PyGame()

    action = game.teacher_action()

    assert action is not None
    assert action in game.legal_actions()


def test_teacher_returns_legal_action_after_move():
    game = pentago_engine.PyGame()

    game.step(game.legal_actions()[0])

    action = game.teacher_action()

    assert action is not None
    assert action in game.legal_actions()

def test_teacher_can_play_complete_game():
    game = pentago_engine.PyGame()

    for _ in range(36):
        if game.game_status() != 0:
            break

        action = game.teacher_action()

        assert action is not None
        assert action in game.legal_actions()

        game.step(action)

    assert game.game_status() != 0
