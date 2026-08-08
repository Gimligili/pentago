from pathlib import Path

import torch
from torch import nn

ACTION_COUNT = 288


class PentagoNet(nn.Module):
    def __init__(self) -> None:
        super().__init__()

        self.features = nn.Sequential(
            nn.Conv2d(
                in_channels=3,
                out_channels=64,
                kernel_size=3,
                padding=1,
            ),
            nn.ReLU(),

            nn.Conv2d(
                in_channels=64,
                out_channels=64,
                kernel_size=3,
                padding=1,
            ),
            nn.ReLU(),
        )

        self.policy_head = nn.Sequential(
            nn.Flatten(),
            nn.Linear(64 * 6 * 6, ACTION_COUNT),
        )

        self.value_head = nn.Sequential(
            nn.Flatten(),
            nn.Linear(64 * 6 * 6, 128),
            nn.ReLU(),
            nn.Linear(128, 1),
            nn.Tanh(),
        )

    def forward(
        self,
        x: torch.Tensor,
    ) -> tuple[torch.Tensor, torch.Tensor]:
        features = self.features(x)

        policy = self.policy_head(features)
        value = self.value_head(features)

        return policy, value

def load_model(
    path: str | Path,
    device: str | torch.device = "cpu",
) -> PentagoNet:
    model = PentagoNet()

    state_dict = torch.load(
        path,
        map_location=device,
        weights_only=True,
    )

    model.load_state_dict(state_dict)
    model.to(device)
    model.eval()

    return model
