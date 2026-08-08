import torch
from pentago_ai.dataset import (
    PentagoTeacherDataset,
    generate_teacher_samples,
)


def test_generate_teacher_samples():
    samples = generate_teacher_samples(
        num_games=2,
        teacher_move_probability=0.5,
    )

    assert len(samples) > 0

    state, action = samples[0]

    assert isinstance(state, torch.Tensor)
    assert state.shape == (3, 6, 6)

    assert isinstance(action, int)
    assert 0 <= action < 288


def test_dataset_wraps_samples():
    samples = generate_teacher_samples(
        num_games=1,
    )

    dataset = PentagoTeacherDataset(samples)

    assert len(dataset) == len(samples)

    state, action = dataset[0]

    assert state.shape == (3, 6, 6)
    assert isinstance(action, int)
def test_dataset_generation_produces_multiple_samples():
    samples = generate_teacher_samples(
        num_games=3,
        teacher_move_probability=0.0,
    )

    assert len(samples) >= 3
