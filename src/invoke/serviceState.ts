export const serviceState = [
  "Stopped",
  "StartPending",
  "StopPending",
  "Running",
  "ContinuePending",
  "PausePending",
  "Paused",
] as const satisfies string[];

export type ServiceState = (typeof serviceState)[number];
