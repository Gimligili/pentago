import torch

from pentago_ai.model import ACTION_COUNT


def mask_illegal_actions(
    logits: torch.Tensor,
    legal_actions: list[int],
) -> torch.Tensor:
    if logits.shape[-1] != ACTION_COUNT:
        raise ValueError(
            f"Expected last dimension to be {ACTION_COUNT}, "
            f"got {logits.shape[-1]}"
        )

    masked_logits = torch.full_like(
        logits,
        float("-inf"),
    )

    if not legal_actions:
        return masked_logits

    legal_indices = torch.tensor(
        legal_actions,
        dtype=torch.long,
        device=logits.device,
    )

    masked_logits[..., legal_indices] = logits[..., legal_indices]

    return masked_logits

def action_probabilities(
    logits: torch.Tensor,
    legal_actions: list[int],
) -> torch.Tensor:
    if not legal_actions:
        return torch.zeros_like(logits)

    masked_logits = mask_illegal_actions(
        logits,
        legal_actions,
    )

    return torch.softmax(masked_logits, dim=-1)