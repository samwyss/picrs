//! constants module
//!
//! container for compile-time constants

/// (F * m^-1) vacuum permittivity https://en.wikipedia.org/wiki/Vacuum_permittivity
pub const VAC_PERM: f64 = 8.8541878188e-12;

/// (F^-1 * m) inverse vacuum permittivity
pub const INV_VAC_PERM: f64 = 1.0 / VAC_PERM;

/// (C) electron charge https://en.wikipedia.org/wiki/Elementary_charge
pub const ELEC_CHARGE: f64 = 1.602176634e-19;

/// (kg) atomic mass unit https://en.wikipedia.org/wiki/Dalton_(unit)
pub const AMU: f64 = 1.66053906892e-27;

/// (kg) electron mass https://en.wikipedia.org/wiki/Electron_mass
pub const ELEC_MASS: f64 = 9.1093837139e-31;

/// (J * K^-1) boltzmann constant https://en.wikipedia.org/wiki/Boltzmann_constant
pub const BOLTZMANN: f64 = 1.380649e-23;

/// (K) electron volt temperature
pub const EV_TEMP: f64 = ELEC_CHARGE / BOLTZMANN;