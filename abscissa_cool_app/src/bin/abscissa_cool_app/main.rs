//! Main entry point for AbscissaCoolApp

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use abscissa_cool_app::application::APPLICATION;

/// Boot AbscissaCoolApp
fn main() {
    abscissa_core::boot(&APPLICATION);
}
