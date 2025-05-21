// This file makes the modules available as a library.

pub mod account;
pub mod bank_statement; // Added as it's used by main_menu
pub mod checking;
pub mod credit;
pub mod customer;
pub mod main_menu;
pub mod person;
pub mod printable; // Added as it might be needed by other modules
pub mod savings;
// run_bank.rs seems to contain a main-like function, so it's probably not part of the library's public API.
// If it contains utility functions needed by the library, they should be refactored.
// For now, omitting run_bank.
