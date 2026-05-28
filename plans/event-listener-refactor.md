# Event Listener Refactor Plan

## Problem

Polling `process.state()` on Windows LLDB is fundamentally unreliable. LLDB's internal
event loop handles DLL-load stops and auto-continues the process before user-space polling
can observe the intermediate `Stopped` state. This causes:

- "process still running" errors when `resume()` is called while LLDB has already re-run the process
- Missed real breakpoint stops because the polling loop cannot distinguish user-visible stops
  from internal LLDB stops

## Solution: SBListener Event Architecture

Use `SBListener::WaitForEvent` + `SBProcess::GetRestartedFromEvent` to observe only
user-visible stop events. LLDB's async mode delivers all state transitions as events;
`GetRestartedFromEvent` tells us whether a `Stopped` event was immediately auto-continued
internally (DLL load, TLS, thread create) vs. a real user-visible stop (breakpoint, step, crash).

## Files to Change

### Layer 1 — C++ Wrappers (`crates/lldb-sys/`)

| File | Change |
|------|--------|
| `wrapper/include/lldb_c.h` | Add `SBBroadcasterRef` typedef + new function declarations |
| `wrapper/src/SBListener.cpp` | Add `LLDB_SBListener_WaitForEvent` |
| `wrapper/src/SBProcess.cpp` | Add `GetBroadcaster`, `GetStateFromEvent`, `GetRestartedFromEvent`, `EventIsProcessEvent` |
| `wrapper/src/SBDebugger.cpp` | Add `LLDB_SBDebugger_GetListener` |
| `wrapper/src/SBBroadcaster.cpp` | NEW — `Destroy`, `IsValid`, `AddListener` |
| `wrapper/src/SBEvent.cpp` | NEW — `Destroy`, `IsValid`, `GetType` |

### Layer 2 — Safe Rust (`crates/lldb-safe/`)

| File | Change |
|------|--------|
| `src/event.rs` | NEW — `Event` wrapper |
| `src/broadcaster.rs` | NEW — `Broadcaster` wrapper |
| `src/listener.rs` | NEW — `Listener` wrapper with `wait_for_event` |
| `src/process.rs` | Add `get_broadcaster()` + 3 static event methods |
| `src/debugger.rs` | Add `get_listener()` |
| `src/lib.rs` | Export new modules |

### Layer 3 — `lldb-native`

| File | Change |
|------|--------|
| `crates/lldb-native/src/thread.rs` | Replace polling with event listener |

## New C++ API

```cpp
// SBListener
bool LLDB_SBListener_WaitForEvent(SBListenerRef ref, uint32_t timeout_secs, SBEventRef* event_out);

// SBProcess
SBBroadcasterRef LLDB_SBProcess_GetBroadcaster(SBProcessRef ref);
LLDBStateType    LLDB_SBProcess_GetStateFromEvent(SBEventRef event);
bool             LLDB_SBProcess_GetRestartedFromEvent(SBEventRef event);
bool             LLDB_SBProcess_EventIsProcessEvent(SBEventRef event);

// SBDebugger
SBListenerRef LLDB_SBDebugger_GetListener(SBDebuggerRef ref);

// SBBroadcaster
void LLDB_SBBroadcaster_Destroy(SBBroadcasterRef ref);
bool LLDB_SBBroadcaster_IsValid(SBBroadcasterRef ref);
bool LLDB_SBBroadcaster_AddListener(SBBroadcasterRef ref, SBListenerRef listener, uint32_t event_mask);

// SBEvent
void     LLDB_SBEvent_Destroy(SBEventRef ref);
bool     LLDB_SBEvent_IsValid(SBEventRef ref);
uint32_t LLDB_SBEvent_GetType(SBEventRef ref);
```

## New thread.rs Architecture

```rust
// 1. Create debugger in async mode
let dbg = Debugger::create(true)?;

// 2. Get the debugger's default listener
let listener = dbg.get_listener()?;

// 3. After target.launch(...), subscribe to process broadcaster
let broadcaster = process.get_broadcaster()?;
broadcaster.add_listener(&listener, 0x01 /* eBroadcastBitStateChanged */)?;

// 4. Replace all polling with wait_for_user_stop():
fn wait_for_user_stop(listener: &Listener, timeout_secs: u32) -> State {
    let deadline = Instant::now() + Duration::from_secs(timeout_secs as u64);
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now()).as_secs() as u32;
        if remaining == 0 { return State::Exited; }
        let event = match listener.wait_for_event(remaining.min(5)) {
            Some(e) => e,
            None => continue,
        };
        if !Process::is_process_event(&event) { continue; }
        let state = Process::state_from_event(&event);
        let restarted = Process::restarted_from_event(&event);
        match state {
            State::Stopped if !restarted => return State::Stopped,
            State::Exited | State::Detached | State::Crashed => return state,
            _ => continue,
        }
    }
}
```

## Key Constants

- `eBroadcastBitStateChanged = 0x00000001` — subscribe to process state changes
