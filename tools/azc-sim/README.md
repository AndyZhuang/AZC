# AZC Industrial Control Simulator

## Overview

The `azc-sim` tool provides simulation and testing capabilities for industrial control systems (SCADA/DCS).

## Features

### 1. PLC Simulation

```ruby
# Define a PLC program
simulator PLC_Tank_Controller
    # Inputs
    sensors:
        level: AnalogInput(0.0..100.0, initial: 50.0)
        temperature: AnalogInput(-20.0..150.0, initial: 25.0)
        pressure: AnalogInput(0.0..10.0, initial: 1.0)
    
    # Outputs
    actuators:
        inlet_valve: DigitalOutput(initial: false)
        outlet_valve: DigitalOutput(initial: false)
        heater: DigitalOutput(initial: false)
        alarm: DigitalOutput(initial: false)
    
    # Control logic
    control:
        if level < 20.0
            inlet_valve = true
        elsif level > 80.0
            inlet_valve = false
        end
        
        if temperature < 18.0
            heater = true
        elsif temperature > 22.0
            heater = false
        end
        
        if pressure > 8.0
            alarm = true
        end
end
```

### 2. Scenario Testing

```ruby
# Define test scenarios
scenarios:
    normal_operation:
        description: "Normal tank operation"
        steps:
            - sensor.level.set(50.0)
            - wait(100ms)
            - assert(inlet_valve.closed?)
            - assert(outlet_valve.closed?)
    
    low_level:
        description: "Low level detection"
        steps:
            - sensor.level.set(15.0)
            - wait(50ms)
            - assert(inlet_valve.open?)
    
    emergency:
        description: "High pressure emergency"
        steps:
            - sensor.pressure.set(9.0)
            - wait(10ms)
            - assert(alarm.active?)
            - assert(inlet_valve.closed?)
            - assert(outlet_valve.open?)
```

### 3. Safety Testing

```ruby
# Safety Integrity Level (SIL) tests
@sil(3)
test emergency_shutdown
    scenario: emergency
    
    # Must complete within 100ms
    deadline: 100ms
    
    # Expected behavior
    expected:
        - alarm.active?
        - all_valves.closed?
        - system.safe?
    
    # Failure modes to test
    failure_modes:
        - sensor_failure
        - actuator_stuck
        - communication_loss
end
```

### 4. Real-time Simulation

```bash
# Run real-time simulation
azc-sim run tank_controller.azc --real-time

# Connect to actual hardware (HIL testing)
azc-sim run tank_controller.azc --hardware /dev/ttyUSB0
```

### 5. Fault Injection

```ruby
# Fault injection testing
fault_injection:
    sensor_drift:
        sensor.level.drift(5.0)  # 5% drift
        assert(controller.detects_fault?)
    
    actuator_stuck:
        inlet_valve.stuck(true)  # Stuck open
        assert(safety_system.activates?)
    
    communication_loss:
        bus.disconnect()
        wait(1000ms)
        assert(failsafe.active?)
```

## Usage

### Basic Simulation

```bash
# Run simulation
azc-sim run controller.azc

# Run specific scenario
azc-sim run controller.azc --scenario low_level

# Run all tests
azc-sim test controller.azc
```

### Report Generation

```bash
# Generate test report
azc-sim report controller.azc --output report.html

# Generate certification report
azc-sim certify controller.azc --sil 3 --output certification.pdf
```

### Interactive Mode

```bash
# Interactive simulation
azc-sim interactive controller.azc

# Commands:
# > set level 50.0
# > get inlet_valve
# > step 100ms
# > assert inlet_valve.open?
```

## Example Output

```
AZC Industrial Control Simulator v0.1.0
======================================

Simulation: Tank_Controller
Duration: 1.0s
Time Step: 10ms

Initial State:
  Sensors:
    level: 50.0
    temperature: 25.0
    pressure: 1.0
  Actuators:
    inlet_valve: false
    outlet_valve: false
    heater: false
    alarm: false

Running simulation...

[100ms] Level: 50.0, Temp: 25.0, Pressure: 1.0
[200ms] Level: 49.8, Temp: 25.0, Pressure: 1.0
[300ms] Injecting fault: pressure = 9.0
[310ms] ⚠️ ALARM TRIGGERED
[310ms] inlet_valve: false
[310ms] outlet_valve: true
[400ms] Fault cleared

Test Results:
✅ normal_operation: PASS (50ms)
✅ low_level: PASS (45ms)
✅ emergency: PASS (10ms)
✅ fault_injection: PASS (100ms)

Safety Score: 98/100
SIL Rating: 3
Certification: ✅ Ready for deployment

Report saved to: tank_controller_report.html
```

## Integration

### CI/CD Pipeline

```yaml
- name: Run Safety Tests
  run: |
    azc-sim test controller.azc
    azc-sim certify controller.azc --sil 3
    
- name: Generate Report
  run: azc-sim report controller.azc --output report.html
```

### Hardware-in-the-Loop (HIL)

```bash
# Connect to real PLC
azc-sim hil controller.azc --device /dev/ttyUSB0 --baud 115200

# Monitor I/O
azc-sim monitor controller.azc --live
```

## Safety Features

1. **Real-time constraints**
   - Deadline monitoring
   - Worst-case execution time analysis

2. **Fault tolerance**
   - Single point of failure analysis
   - Redundancy verification

3. **Formal verification**
   - Model checking
   - Reachability analysis
   - Liveness properties

4. **Certification support**
   - SIL 1-4 compliance
   - IEC 61508 documentation
   - Safety case generation