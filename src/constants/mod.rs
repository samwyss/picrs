//! constants module
//!
//! container for compile-time constants

/// (F * m^-1) vacuum permittivity https://en.wikipedia.org/wiki/Vacuum_permittivity
const VAC_PERM: f64 = 8.8541878188e-12;

/// (C) electron charge https://en.wikipedia.org/wiki/Elementary_charge
const ELEC_CHARGE: f64 = 1.602176634e-19;

/// (kg) atomic mass unit https://en.wikipedia.org/wiki/Dalton_(unit)
const AMU: f64 = 1.66053906892e-27;

/// (kg) electron mass https://en.wikipedia.org/wiki/Electron_mass
const ELEC_MASS: f64 = 9.1093837139e-31;

/// (J * K^-1) boltzmann constant https://en.wikipedia.org/wiki/Boltzmann_constant
const BOLTZMANN: f64 = 1.380649e-23;

/// (K) electron volt temperature
const EV_TEMP: f64 = ELEC_CHARGE / BOLTZMANN;