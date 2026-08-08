import torch
from pentago_ai.model import ACTION_COUNT, PentagoNet


def test_model_output_shape_single_position():
    model = PentagoNet()

    board = torch.zeros(
        1,
        3,
        6,
        6,
        dtype=torch.float32,
    )

    policy, value = model(board)

    assert policy.shape == (1, ACTION_COUNT)
    assert value.shape == (1, 1)


def test_model_output_shape_batch():
    model = PentagoNet()

    boards = torch.zeros(
        8,
        3,
        6,
        6,
        dtype=torch.float32,
    )

    policy, value = model(boards)

    assert policy.shape == (8, ACTION_COUNT)
    assert value.shape == (8, 1)


def test_model_outputs_are_finite():
    model = PentagoNet()

    board = torch.zeros(
        1,
        3,
        6,
        6,
        dtype=torch.float32,
    )

    policy, value = model(board)

    assert torch.isfinite(policy).all()
    assert torch.isfinite(value).all()

def test_value_is_between_minus_one_and_one():
    model = PentagoNet()

    boards = torch.randn(
        16,
        3,
        6,
        6,
    )

    _, value = model(boards)

    assert torch.all(value >= -1.0)
    assert torch.all(value <= 1.0)