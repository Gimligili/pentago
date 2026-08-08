import torch
from pentago_ai.dataset import (
    PentagoTeacherDataset,
    generate_teacher_samples,
)
from pentago_ai.model import PentagoNet
from pentago_ai.training import evaluate_policy, train_policy


def test_training_returns_one_loss_per_epoch():
    samples = generate_teacher_samples(
        num_games=2,
    )

    dataset = PentagoTeacherDataset(
        samples,
    )

    model = PentagoNet()

    history = train_policy(
       model,
       dataset,
       dataset,
       epochs=2,
       batch_size=16,
    )

    assert len(history) == 2

    assert "train_loss" in history[0]
    assert "validation_loss" in history[0]
    assert "top1_accuracy" in history[0]
    assert "top5_accuracy" in history[0]

    assert all(
        torch.isfinite(
            torch.tensor(epoch["train_loss"])
        )
        for epoch in history
    )


def test_training_losses_are_finite():
    samples = generate_teacher_samples(
        num_games=2,
    )

    dataset = PentagoTeacherDataset(
        samples,
    )

    model = PentagoNet()

    history = train_policy(
        model,
        dataset,
        dataset,
        epochs=2,
        batch_size=16,
    )

    assert len(history) == 2

    assert "train_loss" in history[0]
    assert "validation_loss" in history[0]
    assert "top1_accuracy" in history[0]
    assert "top5_accuracy" in history[0]

    assert all(
        torch.isfinite(
            torch.tensor(epoch["train_loss"])
        )
        for epoch in history
    )

def test_training_updates_model_parameters():
    samples = generate_teacher_samples(
        num_games=2,
    )

    dataset = PentagoTeacherDataset(
        samples,
    )

    model = PentagoNet()

    before = {
        name: parameter.detach().clone()
        for name, parameter
        in model.named_parameters()
    }

    _ = train_policy(
        model,
        dataset,
        dataset,
        epochs=2,
        batch_size=16,
    )

    changed = any(
        not torch.equal(
            before[name],
            parameter.detach(),
        )
        for name, parameter
        in model.named_parameters()
    )

    assert changed

def test_evaluate_policy_returns_valid_metrics():
    samples = generate_teacher_samples(
        num_games=2,
    )

    dataset = PentagoTeacherDataset(
        samples,
    )

    model = PentagoNet()

    loss, top1, top5 = evaluate_policy(
        model,
        dataset,
        batch_size=16,
    )

    assert isinstance(loss, float)
    assert isinstance(top1, float)
    assert isinstance(top5, float)

    assert loss >= 0.0

    assert 0.0 <= top1 <= 1.0
    assert 0.0 <= top5 <= 1.0

    assert top5 >= top1
