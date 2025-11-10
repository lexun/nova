# Bevy Debugger Guide

## Overview

bevy_debugger_mcp brings scientific debugging and automated QA to Bevy development. While bevy_brp focuses on real-time manipulation, bevy_debugger focuses on systematic observation, experimentation, and validation.

## What's Enabled

- **Hypothesis-driven debugging** - Form and test hypotheses about game behavior
- **Anomaly detection** - Automatically detect unexpected states or behaviors
- **Performance profiling** - Identify bottlenecks and optimization opportunities
- **Automated testing** - Run stress tests and validate assumptions
- **Replay capabilities** - Record and replay game states for debugging
- **6 specialized scientific debugging tools** via the MCP server

## When to Use Bevy Debugger

### Hypothesis Testing
- You suspect camera movement is causing frame drops
- You think voxel count affects memory usage
- You want to validate rendering assumptions

### Anomaly Detection
- Unexpected entities appearing/disappearing
- Components with invalid values
- Performance degradation over time

### Performance Analysis
- Identifying which systems are slow
- Understanding memory usage patterns
- Optimizing voxel rendering

### Systematic QA
- Validating new features behave as expected
- Regression testing after changes
- Stress testing edge cases

## Tool Categories

### 1. observe
**Purpose**: Systematically observe game state over time

**Use cases**:
- Track entity count as voxel chunks load
- Monitor camera position during movement
- Watch component values change

**Example workflow**:
```
1. Start observation of entity count
2. Load voxel chunks
3. Check if entity count matches expected value
4. If not, investigate discrepancy
```

### 2. experiment
**Purpose**: Run controlled experiments to test hypotheses

**Use cases**:
- Test if increasing voxel density affects FPS
- Verify camera speed changes with different values
- Validate that chunk size impacts memory

**Example workflow**:
```
1. Define hypothesis: "32³ chunks use less memory than 64³ chunks"
2. Set up experiment: measure memory with both sizes
3. Run experiment
4. Analyze results
5. Accept or reject hypothesis
```

### 3. hypothesis
**Purpose**: Formally state and track debugging hypotheses

**Use cases**:
- Document suspected causes of bugs
- Track which hypotheses have been tested
- Build debugging knowledge base

**Example workflow**:
```
1. State hypothesis: "Wireframe rendering causes frame drops"
2. Design experiment to test it
3. Record results
4. Update hypothesis status
```

### 4. detect_anomaly
**Purpose**: Automatically detect unexpected behaviors

**Use cases**:
- Find entities with NaN transforms
- Detect memory leaks
- Identify infinite loops or runaway systems

**Example workflow**:
```
1. Define normal behavior baseline
2. Run game
3. Automatically detect deviations
4. Alert when anomalies occur
```

### 5. stress_test
**Purpose**: Push systems to their limits to find breaking points

**Use cases**:
- Load thousands of voxel chunks
- Spam input events
- Create extreme camera movements

**Example workflow**:
```
1. Define stress scenario: "Load 100 chunks simultaneously"
2. Run stress test
3. Monitor for crashes, slowdowns, or errors
4. Document findings
```

### 6. replay
**Purpose**: Record and replay game states for reproducible debugging

**Use cases**:
- Capture bug reproduction steps
- Share exact game state with team
- Test fixes against recorded scenarios

**Example workflow**:
```
1. Start recording game state
2. Reproduce bug
3. Stop recording
4. Replay to verify bug occurs consistently
5. Apply fix and replay to verify fix
```

## Comparison: BRP vs Debugger

| Use Case | Tool to Use |
|----------|-------------|
| Change voxel color to see if it looks better | **BRP** (direct manipulation) |
| Test if voxel count affects performance | **Debugger** (hypothesis testing) |
| Take screenshot of current scene | **BRP** (visual verification) |
| Detect memory leaks over time | **Debugger** (anomaly detection) |
| Add a test entity to the scene | **BRP** (entity manipulation) |
| Validate camera movement is correct | **Debugger** (observation & validation) |
| Simulate keyboard input | **BRP** (input simulation) |
| Stress test chunk loading | **Debugger** (stress testing) |

## Scientific Debugging Methodology

Bevy Debugger encourages a systematic approach:

1. **Observe** - Gather data about the problem
2. **Hypothesize** - Form testable explanations
3. **Experiment** - Design controlled tests
4. **Analyze** - Interpret results
5. **Conclude** - Accept/reject hypothesis
6. **Iterate** - Refine understanding

This is more rigorous than ad-hoc debugging and leads to deeper understanding.

## Current Use Cases in Nova

### Voxel System Validation

**Question**: Are voxels being created at the correct positions?

**Approach**:
```
1. Observe: Count voxel entities (should be ~50 for our test chunk)
2. Hypothesis: "Voxels are positioned correctly relative to chunk_offset"
3. Experiment: Query transform components and verify coordinates
4. Analyze: Compare actual vs expected positions
5. Conclude: Accept or reject positioning logic
```

### Performance Testing

**Question**: How many voxel entities can we render before FPS drops?

**Approach**:
```
1. Stress test: Incrementally add voxel chunks
2. Observe: Monitor FPS at each step
3. Detect anomaly: Alert when FPS drops below 60
4. Conclude: Establish performance budget
```

### Camera System Verification

**Question**: Does camera movement match expected physics?

**Approach**:
```
1. Observe: Record camera transform over time
2. Hypothesis: "Camera velocity is constant at move_speed"
3. Experiment: Measure distance traveled per second
4. Analyze: Compare to move_speed value (5.0)
5. Conclude: Verify or debug movement logic
```

## Integration with Current Nova Code

bevy_debugger_mcp is an **external MCP server** - it doesn't require a Bevy plugin. It works by:

1. Connecting to BRP (port 15702) to query game state
2. Running analysis and experiments using BRP data
3. Providing higher-level debugging tools

The example is already configured correctly with BRP, which is all that's needed.

## Validation Tests

Once the MCP servers are installed, run these tests to verify bevy_debugger is working:

1. **Observation test** - Observe entity count over 10 seconds
2. **Hypothesis test** - State and track a simple hypothesis
3. **Experiment test** - Run experiment to measure voxel count
4. **Anomaly detection test** - Detect if any entities have invalid transforms
5. **Stress test** - Load multiple chunks and monitor performance

## Best Practices

### When to Form Hypotheses
- Before making changes ("this component affects performance")
- When debugging ("the bug is caused by X")
- When optimizing ("technique Y will improve FPS")

### When to Use Stress Tests
- Before deploying new features
- After performance optimizations
- When testing edge cases

### When to Record Replays
- When you find a bug (capture reproduction)
- When testing fixes (verify behavior)
- When documenting issues (share with team)

## Workflow Example: Debug Voxel Positioning Issue

Let's say we notice voxels aren't appearing where we expect:

```
1. Observe: Query all voxel entities and their transforms
2. Hypothesis: "chunk_offset calculation is incorrect"
3. Experiment:
   - Calculate expected position for voxel at (10,0,10)
   - Query actual position from game
   - Compare values
4. Analyze:
   - If positions match → hypothesis rejected
   - If positions differ → hypothesis supported
5. If supported:
   - Fix chunk_offset calculation
   - Re-run experiment to verify fix
   - Update code
```

## Next Steps

1. Install MCP servers: `install-mcp-servers`
2. Restart Claude Code conversation to load new tools
3. Run the 5 validation tests above
4. Try the voxel positioning workflow above
5. Document findings and refine debugging methodology

## References

- bevy_debugger_mcp: Scientific debugging MCP server for Bevy
- Bevy Remote Protocol: The underlying protocol for querying game state
- Scientific Method: Hypothesis testing applied to software debugging
