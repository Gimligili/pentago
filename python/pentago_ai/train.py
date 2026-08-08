from pathlib import Path

from pentago_ai.dataset import (
    PentagoTeacherDataset,
    generate_teacher_samples,
)
from pentago_ai.model import PentagoNet
from pentago_ai.training import train_policy

MODEL_DIR = Path("models")
MODEL_PATH = MODEL_DIR / "pentago_policy.pt"


def main() -> None:
    print("Generating teacher dataset...")

    training_samples = generate_teacher_samples(
        num_games=800,
        teacher_move_probability=0.5,
    )

    validation_samples = generate_teacher_samples(
        num_games=200,
        teacher_move_probability=0.5,
    )

    train_dataset = PentagoTeacherDataset(training_samples)
    validation_dataset = PentagoTeacherDataset(validation_samples)

    print(f"Training samples: {len(train_dataset)}")
    print(f"Validation samples: {len(validation_dataset)}")

    model = PentagoNet()

    MODEL_DIR.mkdir(
        parents=True,
        exist_ok=True,
    )

    print("Training...")

    history = train_policy(
        model,
        train_dataset,
        validation_dataset,
        epochs=10,
        batch_size=128,
        learning_rate=1e-3,
        checkpoint_path=MODEL_PATH,
    )

    for metrics in history:
        print(
            f"Epoch {metrics['epoch']:02d} | "
            f"train_loss={metrics['train_loss']:.4f} | "
            f"val_loss={metrics['validation_loss']:.4f} | "
            f"top1={metrics['top1_accuracy'] * 100:.2f}% | "
            f"top5={metrics['top5_accuracy'] * 100:.2f}%"
        )

    best_metrics = min(
        history,
        key=lambda metrics: metrics["validation_loss"],
    )

    print(
        f"Best epoch: {best_metrics['epoch']:02d} | "
        f"val_loss={best_metrics['validation_loss']:.4f} | "
        f"top1={best_metrics['top1_accuracy'] * 100:.2f}% | "
        f"top5={best_metrics['top5_accuracy'] * 100:.2f}%"
    )


if __name__ == "__main__":
    main()