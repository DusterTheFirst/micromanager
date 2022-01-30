#![forbid(unsafe_code)]

/// Control remote devices
/// 
/// Data returned from a telecommand differs from [`telemetry`](crate::telemetry)
/// as its value is tied to an action taken on the commands behalf and the result
/// being lost will cause a de-sync of state between the commander and the commanded
pub mod telecommand;

/// Collection of metrics from remote devices, often superfluous
/// 
/// Telemetry data is meant to be interpreted as stand alone packets and should
/// not rely on any previous data. Telemetry data must be treated as if it is
/// always transmitted over a lossy medium, and every other packet has been lost.
pub mod telemetry;
