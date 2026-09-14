/*
 * Nexus Browser
 *
 * Copyright (C) 2026 Nexus Browser Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 *
 * See LICENSE for details.
 */

mod browser_kernel;
mod browser_core;

mod engines;

mod privacy;
mod security;
mod networking;
mod dns;

mod storage;
mod profiles;
mod workspaces;

mod extensions;
mod benchmarks;

mod developer_tools;

use anyhow::Result;

use browser_kernel::kernel_core::BrowserKernel;
use browser_kernel::logging_system::LoggingSystem;


fn main() -> Result<()> {

    /*
        Initialize logging first.
    */

    LoggingSystem::initialize();


    log::info!("Starting Nexus Browser");


    /*
        Create browser kernel.

        The kernel controls:

        - engine management
        - security policies
        - privacy policies
        - resources
        - updates
        - benchmarking
    */

    let mut kernel = BrowserKernel::new();


    /*
        Initialize core systems.
    */

    kernel.initialize()?;


    log::info!(
        "Nexus Browser kernel initialized"
    );


    /*
        Start runtime.

        Future versions will launch:

        - UI subsystem
        - workspace manager
        - MicroVM manager
        - rendering engines
        - network policies
    */

    kernel.start()?;


    log::info!(
        "Nexus Browser running"
    );


    /*
        Main event loop.

        The browser remains alive until
        shutdown is requested.
    */

    kernel.run()?;


    log::info!(
        "Nexus Browser shutting down"
    );


    Ok(())
}
