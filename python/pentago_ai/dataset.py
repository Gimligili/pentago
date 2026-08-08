import random

import torch
from torch.utils.data import Dataset

from pentago_ai import pentago_engine
from pentago_ai.encoding import encode_board


class PentagoTeacherDataset(Dataset):
    def __init__(
        self,
        samples: list[tuple[torch.Tensor, int]],
    ) -> None:
        self.samples = samples

    def __len__(self) -> int:
        return len(self.samples)

    def __getitem__(
        self,
        index: int,
    ) -> tuple[torch.Tensor, int]:
        return self.samples[index]


def generate_teacher_samples(
    num_games: int,
    teacher_move_probability: float = 0.5,
) -> list[tuple[torch.Tensor, int]]:
    samples: list[tuple[torch.Tensor, int]] = []

    for _ in range(num_games):
        game = pentago_engine.PyGame()

        while game.game_status() == 0:
            teacher_action = game.teacher_action()

            if teacher_action is None:
                break

            state = encode_board(game)

            samples.append(
                (
                    state.clone(),
                    teacher_action,
                )
            )

            legal_actions = game.legal_actions()

            if random.random() < teacher_move_probability:
                action = teacher_action
            else:
                action = random.choice(legal_actions)

            game.step(action)

    return samples