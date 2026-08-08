import torch
from pentago_ai.model import ACTION_COUNT
from pentago_ai.policy import (
    action_probabilities,
    mask_illegal_actions,
)


def test_illegal_actions_are_masked():
    logits = torch.zeros(ACTION_COUNT)

    legal_actions = [0, 10, 287]

    masked = mask_illegal_actions(
        logits,
        legal_actions,
    )

    assert masked[0] == 0
    assert masked[10] == 0
    assert masked[287] == 0

    assert torch.isneginf(masked[1])
    assert torch.isneginf(masked[100])


def test_probabilities_sum_to_one():
    logits = torch.zeros(ACTION_COUNT)

    probabilities = action_probabilities(
        logits,
        [0, 1, 2],
    )

    assert torch.isclose(
        probabilities.sum(),
        torch.tensor(1.0),
    )


def test_illegal_action_probability_is_zero():
    logits = torch.randn(ACTION_COUNT)

    probabilities = action_probabilities(
        logits,
        [5, 42],
    )

    assert probabilities[0] == 0
    assert probabilities[100] == 0

    assert probabilities[5] > 0
    assert probabilities[42] > 0

def test_no_legal_actions_returns_zero_probabilities():
    logits = torch.randn(ACTION_COUNT)

    probabilities = action_probabilities(
        logits,
        [],
    )

    assert torch.all(probabilities == 0)
