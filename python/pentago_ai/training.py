from pathlib import Path

import torch
from torch import nn
from torch.utils.data import DataLoader, Dataset

from pentago_ai.model import PentagoNet


def train_policy(
    model: PentagoNet,
    train_dataset: Dataset,
    validation_dataset: Dataset,
    *,
    epochs: int = 10,
    batch_size: int = 128,
    learning_rate: float = 1e-3,
    device: str | torch.device = "cpu",
    checkpoint_path: str | Path | None = None,
) -> list[dict[str, float]]:
    model = model.to(device)

    train_loader = DataLoader(
        train_dataset,
        batch_size=batch_size,
        shuffle=True,
    )

    optimizer = torch.optim.Adam(
        model.parameters(),
        lr=learning_rate,
    )

    criterion = nn.CrossEntropyLoss()

    history: list[dict[str, float]] = []

    best_validation_loss = float("inf")

    for epoch in range(1, epochs + 1):
        model.train()

        total_loss = 0.0
        total_samples = 0

        for states, actions in train_loader:
            states = states.to(device)
            actions = actions.to(device)

            optimizer.zero_grad()

            policy_logits, _ = model(states)

            loss = criterion(
                policy_logits,
                actions,
            )

            loss.backward()
            optimizer.step()

            batch_size_actual = states.shape[0]

            total_loss += loss.item() * batch_size_actual
            total_samples += batch_size_actual

        train_loss = total_loss / total_samples

        validation_loss, top1, top5 = evaluate_policy(
            model,
            validation_dataset,
            batch_size=batch_size,
            device=device,
        )

        if validation_loss < best_validation_loss:
            best_validation_loss = validation_loss

            if checkpoint_path is not None:
                torch.save(model.state_dict(),checkpoint_path)

        history.append(
            {
                "epoch": epoch,
                "train_loss": train_loss,
                "validation_loss": validation_loss,
                "top1_accuracy": top1,
                "top5_accuracy": top5,
            }
        )

    return history

def evaluate_policy(
    model: PentagoNet,
    dataset: Dataset,
    *,
    batch_size: int = 128,
    device: str | torch.device = "cpu",
) -> tuple[float, float, float]:
    model = model.to(device)
    model.eval()

    dataloader = DataLoader(
        dataset,
        batch_size=batch_size,
        shuffle=False,
    )

    criterion = nn.CrossEntropyLoss()

    total_loss = 0.0
    total_samples = 0
    top1_correct = 0
    top5_correct = 0

    with torch.no_grad():
        for states, actions in dataloader:
            states = states.to(device)
            actions = actions.to(device)

            policy_logits, _ = model(states)

            loss = criterion(
                policy_logits,
                actions,
            )

            batch_size_actual = states.shape[0]

            total_loss += (
                loss.item() * batch_size_actual
            )

            total_samples += batch_size_actual

            # Top-1 accuracy
            predictions = policy_logits.argmax(dim=1)

            top1_correct += (
                predictions == actions
            ).sum().item()

            # Top-5 accuracy
            top5_predictions = policy_logits.topk(
                k=5,
                dim=1,
            ).indices

            top5_correct += (
                top5_predictions == actions.unsqueeze(1)
            ).any(dim=1).sum().item()

    average_loss = total_loss / total_samples
    top1_accuracy = top1_correct / total_samples
    top5_accuracy = top5_correct / total_samples

    return (
        average_loss,
        top1_accuracy,
        top5_accuracy,
    )