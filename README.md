# Plant Waterer

A reboot-safe, observable, automatic plant watering system.

## Overview

A Raspberry Pi waters a plant on a fixed schedule by mixing water in a reservoir then pumping that water into the plant pot. The software implements a persistent state machine, an error logging framework, mocking capabilities, as well as abstraction between controlling logic and hardware.

It follows a run-to-completion task model:
1. The operating system schedules execution.
2. The program performs exactly one watering cycle.
3. Progress is persisted to disk.
4. The program exits.

There is no long-running event loop and no internal scheduler, ensuring robustness across reboots and failures.

The following is the workflow to activate the system:

```
# In root directory:

./install/build-and-install.sh  # Build and install the main program

./install/install-systemd.sh  # Install the scheduler daemon
```

## Hardware Design

The following are the core hardware components:
- A Raspberry Pi, running the Linux-based Raspberry Pi OS
- A 12 V DC motor, along with a shaft and propeller blades for mixing
- A 12 V DC solenoid valve and tubing for pumping water
- A relay module for actuator isolation and control via GPIO
- A shared 12 V power supply

The Raspberry Pi supplies 3.3 V logic signals to control a relay module with two relays: one for mixing and one for watering. When a given relay is switched on, 12 V power flows through the corresponding actuator. A relay was chosen as the method of galvanic isolation instead of a transistor for simplicity; a transistor's low current draw and high switching speed are not needed due to the system's low duty cycle being on the order of days or weeks.

Upon cycle start, the mixing motor is activated. The motor is connected via a shaft to propellor blades which are submerged in the reservoir, mixing the water. Mixing is necessary because, when the system is idle, dissolved fertilizer in the reservoir stratifies (but does necessarily precipitate), and plants are generally poorly adapted to mineral concentration gradients.

When mixing is complete, the solenoid valve is opened. The valve is connected on one end to a tube submerged in the reservoir and connected on the other with a tube leading to the plant pot. The former is kept perpetually full with water and, with the reservoir placed higher than the plant pot, the pressure is held by gravity. When the valve opens, water siphons from the reservoir into the plant pot.

## Software Design

### Scheduling Strategy

Scheduling is delegated to the operating system, not implemented in application code. `systemd` timers handle timekeeping, reboots, and missed runs, all of which would be fragile if implement manually. The application itself has no concept of “every N days”; it simply performs one cycle when invoked.

### State Management and Reboot Safety

The system must behave correctly if power is lost mid-cycle, the process is terminated, or the system reboots unexpectedly. To this end, an explicit persistent state is stored on disk with one of the following variants: `Idle`, `MixingStarted`, `MixingCompleted`, `WateringStarted`, and `Completed`. State transitions are written before actions occur. On startup, the program turns off all actuators, loads the last known state, then safely resumes or restarts steps as needed.

### Shutdown and Signal Handling

Hardware control is single-threaded. This allows the signal handler to function without needing to acquire the lock of a mutex to turn off the actuators. Instead, the signal handler creates an atomic shutdown flag, then handles termination signals by activating the flag. During the run cycle, the controller interrogates the signal handler frequently and, if it finds that the flag is activated, the same controller shuts down the actuators before exiting safely, avoiding a deadlock.

### Error Handling and Logging

To provide post-mortem visibility, logs are written to `stdout` and `stderr` using the `log` crate, while the `systemd` journal is used as the log store. The hardware layer logs actuation failures, the controller logs step-level failures, and the main function logs fatal termination only once. Normal shutdown is logged as an operator event rather than an error.

Logs can be reviewed with the following command:

```
journalctl -u plantwaterer.service
```

### Mocking Capabilities and Abstraction

Two GPIO backends are provided:
1. A real hardware backend based on the `rppal` crate, intended for execution on a Raspberry Pi.
2. A mock backend that replaces GPIO operations with `println!` statements, intended for alternative Linux-system development, debugging, and CI environments.

Both backends implement the same trait interface and expose identical constructor semantics. A single concrete type, `PlatformPin`, is exported from the module, mapped internally to the backend implementation selected at compile time.

The following command invokes the mock GPIO implementation:

```
cargo run --features gpio-mock --no-default-features
```
