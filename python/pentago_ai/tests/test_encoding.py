import torch
from pentago_ai import pentago_engine
from pentago_ai.encoding import encode_board


def test_empty_board_encoding_shape():
    game = pentago_engine.PyGame()

    encoded = encode_board(game)

    assert encoded.shape == (3, 6, 6)
    assert encoded.dtype == torch.float32


def test_empty_board_encoding_content():
    game = pentago_engine.PyGame()

    encoded = encode_board(game)

    current = encoded[0]
    opponent = encoded[1]
    empty = encoded[2]

    assert torch.count_nonzero(current) == 0
    assert torch.count_nonzero(opponent) == 0
    assert torch.all(empty == 1)


def test_encoding_is_from_current_player_perspective():
    game = pentago_engine.PyGame()

    action = game.legal_actions()[0]
    game.step(action)

    # After White has played, Black is the current player.
    assert game.current_player() == -1

    encoded = encode_board(game)

    current = encoded[0]
    opponent = encoded[1]
    empty = encoded[2]

    # Black has not placed anything yet.
    assert torch.count_nonzero(current) == 0

    # White has exactly one marble on the board.
    assert torch.count_nonzero(opponent) == 1

    # 35 cells remain empty.
    assert torch.count_nonzero(empty) == 35