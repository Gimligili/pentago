import numpy as np
import torch


def encode_board(game) -> torch.Tensor:
    board = np.asarray(game.board(), dtype=np.int8)
    current_player = game.current_player()

    current = board == current_player
    opponent = board == -current_player
    empty = board == 0

    encoded = np.stack(
        [current, opponent, empty],
        axis=0,
    ).astype(np.float32)

    return torch.from_numpy(encoded)