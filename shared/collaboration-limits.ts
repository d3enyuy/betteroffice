/**
 * Largest collaboration frame the relay accepts, broadcasts and retains.
 * Clients must not exceed it: the relay cannot replay what it cannot retain.
 */
export const MAX_COLLABORATION_FRAME_BYTES = 16 * 1024 * 1024;

/**
 * Total document history retained per room. Four times the single-frame cap so
 * one maximum-size update trims the oldest entries instead of evicting everything.
 */
export const MAX_RETAINED_HISTORY_BYTES = 64 * 1024 * 1024;

/** Upper bound on the bytes replayed to one joining socket. */
export const MAX_JOIN_REPLAY_BYTES = MAX_RETAINED_HISTORY_BYTES;
