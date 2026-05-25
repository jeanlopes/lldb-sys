#ifndef LLDB_C_H
#define LLDB_C_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* =========================================================================
 * Opaque handle types
 * Every SB object lives on the heap; callers own the handle and must call
 * the corresponding _Destroy function to free it.
 * ========================================================================= */
typedef void* SBDebuggerRef;
typedef void* SBTargetRef;
typedef void* SBProcessRef;
typedef void* SBThreadRef;
typedef void* SBFrameRef;
typedef void* SBBreakpointRef;
typedef void* SBBreakpointLocationRef;
typedef void* SBValueRef;
typedef void* SBErrorRef;
typedef void* SBFileSpecRef;
typedef void* SBLineEntryRef;
typedef void* SBAddressRef;
typedef void* SBSymbolRef;
typedef void* SBModuleRef;
typedef void* SBListenerRef;
typedef void* SBEventRef;

/* =========================================================================
 * Enumerations (mirror lldb-enumerations.h)
 * ========================================================================= */
typedef enum {
    LLDB_STATE_INVALID    = 0,
    LLDB_STATE_UNLOADED   = 1,
    LLDB_STATE_CONNECTED  = 2,
    LLDB_STATE_ATTACHING  = 3,
    LLDB_STATE_LAUNCHING  = 4,
    LLDB_STATE_STOPPED    = 5,
    LLDB_STATE_RUNNING    = 6,
    LLDB_STATE_STEPPING   = 7,
    LLDB_STATE_CRASHED    = 8,
    LLDB_STATE_DETACHED   = 9,
    LLDB_STATE_EXITED     = 10,
    LLDB_STATE_SUSPENDED  = 11,
} LLDBStateType;

typedef enum {
    LLDB_STOP_REASON_INVALID            = 0,
    LLDB_STOP_REASON_NONE               = 1,
    LLDB_STOP_REASON_TRACE              = 2,
    LLDB_STOP_REASON_BREAKPOINT         = 3,
    LLDB_STOP_REASON_WATCHPOINT         = 4,
    LLDB_STOP_REASON_SIGNAL             = 5,
    LLDB_STOP_REASON_EXCEPTION          = 6,
    LLDB_STOP_REASON_EXEC               = 7,
    LLDB_STOP_REASON_PLAN_COMPLETE      = 8,
    LLDB_STOP_REASON_THREAD_EXITING     = 9,
    LLDB_STOP_REASON_INSTRUMENTATION    = 10,
    LLDB_STOP_REASON_PROCESSOR_TRACE    = 11,
    LLDB_STOP_REASON_FORK               = 12,
    LLDB_STOP_REASON_VFORK              = 13,
    LLDB_STOP_REASON_VFORK_DONE         = 14,
} LLDBStopReason;

typedef enum {
    LLDB_LAUNCH_FLAG_NONE                        = 0,
    LLDB_LAUNCH_FLAG_EXEC                        = (1 << 0),
    LLDB_LAUNCH_FLAG_DEBUG                       = (1 << 1),
    LLDB_LAUNCH_FLAG_STOP_AT_ENTRY               = (1 << 2),
    LLDB_LAUNCH_FLAG_DISABLE_ASLR                = (1 << 3),
    LLDB_LAUNCH_FLAG_DISABLE_STDIO               = (1 << 4),
    LLDB_LAUNCH_FLAG_LAUNCH_IN_TTY               = (1 << 5),
    LLDB_LAUNCH_FLAG_LAUNCH_IN_SHELL             = (1 << 6),
    LLDB_LAUNCH_FLAG_LAUNCH_IN_SEPARATE_PROCESS_GROUP = (1 << 7),
    LLDB_LAUNCH_FLAG_DONT_SET_EXIT_STATUS        = (1 << 8),
    LLDB_LAUNCH_FLAG_DETACH_ON_ERROR             = (1 << 9),
    LLDB_LAUNCH_FLAG_SHELL_EXPAND_ARGS           = (1 << 10),
    LLDB_LAUNCH_FLAG_CLOSE_TTY_ON_EXIT           = (1 << 11),
    LLDB_LAUNCH_FLAG_INHERIT_FROM_PARENT         = (1 << 12),
} LLDBLaunchFlags;

/* =========================================================================
 * SBDebugger
 * ========================================================================= */
void          LLDB_SBDebugger_Initialize(void);
void          LLDB_SBDebugger_Terminate(void);
SBDebuggerRef LLDB_SBDebugger_Create(bool async);
void          LLDB_SBDebugger_Destroy(SBDebuggerRef ref);
bool          LLDB_SBDebugger_IsValid(SBDebuggerRef ref);
void          LLDB_SBDebugger_SetAsync(SBDebuggerRef ref, bool async);
bool          LLDB_SBDebugger_GetAsync(SBDebuggerRef ref);
SBTargetRef   LLDB_SBDebugger_CreateTarget(SBDebuggerRef ref,
                                            const char* filename,
                                            const char* triple,
                                            const char* platform,
                                            bool        add_deps,
                                            SBErrorRef  error);
SBTargetRef   LLDB_SBDebugger_CreateTargetSimple(SBDebuggerRef ref, const char* filename);
uint32_t      LLDB_SBDebugger_GetNumTargets(SBDebuggerRef ref);
SBTargetRef   LLDB_SBDebugger_GetTargetAtIndex(SBDebuggerRef ref, uint32_t idx);
SBTargetRef   LLDB_SBDebugger_GetSelectedTarget(SBDebuggerRef ref);
const char*   LLDB_SBDebugger_GetVersionString(void);

/* =========================================================================
 * SBError
 * ========================================================================= */
SBErrorRef  LLDB_SBError_Create(void);
void        LLDB_SBError_Destroy(SBErrorRef ref);
bool        LLDB_SBError_IsValid(SBErrorRef ref);
bool        LLDB_SBError_Success(SBErrorRef ref);
bool        LLDB_SBError_Fail(SBErrorRef ref);
const char* LLDB_SBError_GetCString(SBErrorRef ref);
uint32_t    LLDB_SBError_GetError(SBErrorRef ref);
void        LLDB_SBError_SetErrorString(SBErrorRef ref, const char* err_str);
void        LLDB_SBError_Clear(SBErrorRef ref);

/* =========================================================================
 * SBTarget
 * ========================================================================= */
void          LLDB_SBTarget_Destroy(SBTargetRef ref);
bool          LLDB_SBTarget_IsValid(SBTargetRef ref);
SBProcessRef  LLDB_SBTarget_LaunchSimple(SBTargetRef ref,
                                          const char** argv,
                                          const char** envp,
                                          const char*  working_dir);
SBProcessRef  LLDB_SBTarget_Launch(SBTargetRef    ref,
                                    SBListenerRef  listener,
                                    const char**   argv,
                                    const char**   envp,
                                    const char*    stdin_path,
                                    const char*    stdout_path,
                                    const char*    stderr_path,
                                    const char*    working_dir,
                                    uint32_t       launch_flags,
                                    bool           stop_at_entry,
                                    SBErrorRef     error);
SBProcessRef  LLDB_SBTarget_AttachToProcessWithID(SBTargetRef   ref,
                                                   SBListenerRef listener,
                                                   uint64_t      pid,
                                                   SBErrorRef    error);
SBProcessRef  LLDB_SBTarget_GetProcess(SBTargetRef ref);
SBBreakpointRef LLDB_SBTarget_BreakpointCreateByName(SBTargetRef ref,
                                                      const char* name,
                                                      const char* module_name);
SBBreakpointRef LLDB_SBTarget_BreakpointCreateByLocation(SBTargetRef ref,
                                                          const char* file,
                                                          uint32_t    line);
SBBreakpointRef LLDB_SBTarget_BreakpointCreateByAddress(SBTargetRef ref, uint64_t address);
bool            LLDB_SBTarget_DeleteBreakpoint(SBTargetRef ref, uint32_t break_id);
uint32_t        LLDB_SBTarget_GetNumBreakpoints(SBTargetRef ref);
SBBreakpointRef LLDB_SBTarget_GetBreakpointAtIndex(SBTargetRef ref, uint32_t idx);
const char*     LLDB_SBTarget_GetTriple(SBTargetRef ref);

/* =========================================================================
 * SBProcess
 * ========================================================================= */
void          LLDB_SBProcess_Destroy(SBProcessRef ref);
bool          LLDB_SBProcess_IsValid(SBProcessRef ref);
LLDBStateType LLDB_SBProcess_GetState(SBProcessRef ref);
uint64_t      LLDB_SBProcess_GetProcessID(SBProcessRef ref);
uint32_t      LLDB_SBProcess_GetUniqueID(SBProcessRef ref);
SBErrorRef    LLDB_SBProcess_Continue(SBProcessRef ref);
SBErrorRef    LLDB_SBProcess_Stop(SBProcessRef ref);
SBErrorRef    LLDB_SBProcess_Kill(SBProcessRef ref);
SBErrorRef    LLDB_SBProcess_Detach(SBProcessRef ref);
uint32_t      LLDB_SBProcess_GetNumThreads(SBProcessRef ref);
SBThreadRef   LLDB_SBProcess_GetThreadAtIndex(SBProcessRef ref, size_t idx);
SBThreadRef   LLDB_SBProcess_GetThreadByID(SBProcessRef ref, uint64_t tid);
SBThreadRef   LLDB_SBProcess_GetThreadByIndexID(SBProcessRef ref, uint32_t idx_id);
SBThreadRef   LLDB_SBProcess_GetSelectedThread(SBProcessRef ref);
bool          LLDB_SBProcess_SetSelectedThreadByID(SBProcessRef ref, uint64_t tid);
int32_t       LLDB_SBProcess_GetExitStatus(SBProcessRef ref);
const char*   LLDB_SBProcess_GetExitDescription(SBProcessRef ref);
size_t        LLDB_SBProcess_ReadMemory(SBProcessRef ref,
                                         uint64_t     addr,
                                         void*        buf,
                                         size_t       size,
                                         SBErrorRef   error);
size_t        LLDB_SBProcess_WriteMemory(SBProcessRef ref,
                                          uint64_t     addr,
                                          const void*  buf,
                                          size_t       size,
                                          SBErrorRef   error);

/* =========================================================================
 * SBThread
 * ========================================================================= */
void          LLDB_SBThread_Destroy(SBThreadRef ref);
bool          LLDB_SBThread_IsValid(SBThreadRef ref);
uint64_t      LLDB_SBThread_GetThreadID(SBThreadRef ref);
uint32_t      LLDB_SBThread_GetIndexID(SBThreadRef ref);
const char*   LLDB_SBThread_GetName(SBThreadRef ref);
const char*   LLDB_SBThread_GetQueueName(SBThreadRef ref);
LLDBStopReason LLDB_SBThread_GetStopReason(SBThreadRef ref);
size_t        LLDB_SBThread_GetStopReasonDataCount(SBThreadRef ref);
uint64_t      LLDB_SBThread_GetStopReasonDataAtIndex(SBThreadRef ref, uint32_t idx);
const char*   LLDB_SBThread_GetStopDescription(SBThreadRef ref, char* dst, size_t dst_len);
uint32_t      LLDB_SBThread_GetNumFrames(SBThreadRef ref);
SBFrameRef    LLDB_SBThread_GetFrameAtIndex(SBThreadRef ref, uint32_t idx);
SBFrameRef    LLDB_SBThread_GetSelectedFrame(SBThreadRef ref);
void          LLDB_SBThread_StepOver(SBThreadRef ref);
void          LLDB_SBThread_StepInto(SBThreadRef ref);
void          LLDB_SBThread_StepOut(SBThreadRef ref);
void          LLDB_SBThread_StepInstruction(SBThreadRef ref, bool step_over);
void          LLDB_SBThread_RunToAddress(SBThreadRef ref, uint64_t addr);

/* =========================================================================
 * SBFrame
 * ========================================================================= */
void        LLDB_SBFrame_Destroy(SBFrameRef ref);
bool        LLDB_SBFrame_IsValid(SBFrameRef ref);
uint32_t    LLDB_SBFrame_GetFrameID(SBFrameRef ref);
uint64_t    LLDB_SBFrame_GetCFA(SBFrameRef ref);
uint64_t    LLDB_SBFrame_GetPC(SBFrameRef ref);
bool        LLDB_SBFrame_SetPC(SBFrameRef ref, uint64_t new_pc);
uint64_t    LLDB_SBFrame_GetSP(SBFrameRef ref);
uint64_t    LLDB_SBFrame_GetFP(SBFrameRef ref);
const char* LLDB_SBFrame_GetFunctionName(SBFrameRef ref);
bool        LLDB_SBFrame_IsInlined(SBFrameRef ref);
bool        LLDB_SBFrame_IsArtificial(SBFrameRef ref);
SBValueRef  LLDB_SBFrame_FindVariable(SBFrameRef ref, const char* name);
SBValueRef  LLDB_SBFrame_EvaluateExpression(SBFrameRef ref, const char* expr);

/* =========================================================================
 * SBBreakpoint
 * ========================================================================= */
void        LLDB_SBBreakpoint_Destroy(SBBreakpointRef ref);
bool        LLDB_SBBreakpoint_IsValid(SBBreakpointRef ref);
uint32_t    LLDB_SBBreakpoint_GetID(SBBreakpointRef ref);
void        LLDB_SBBreakpoint_SetEnabled(SBBreakpointRef ref, bool enable);
bool        LLDB_SBBreakpoint_IsEnabled(SBBreakpointRef ref);
bool        LLDB_SBBreakpoint_IsOneShot(SBBreakpointRef ref);
void        LLDB_SBBreakpoint_SetOneShot(SBBreakpointRef ref, bool one_shot);
uint32_t    LLDB_SBBreakpoint_GetHitCount(SBBreakpointRef ref);
void        LLDB_SBBreakpoint_ResetHitCount(SBBreakpointRef ref);
void        LLDB_SBBreakpoint_SetCondition(SBBreakpointRef ref, const char* condition);
const char* LLDB_SBBreakpoint_GetCondition(SBBreakpointRef ref);
void        LLDB_SBBreakpoint_SetIgnoreCount(SBBreakpointRef ref, uint32_t count);
uint32_t    LLDB_SBBreakpoint_GetIgnoreCount(SBBreakpointRef ref);
uint32_t    LLDB_SBBreakpoint_GetNumLocations(SBBreakpointRef ref);

/* =========================================================================
 * SBValue
 * ========================================================================= */
void        LLDB_SBValue_Destroy(SBValueRef ref);
bool        LLDB_SBValue_IsValid(SBValueRef ref);
const char* LLDB_SBValue_GetName(SBValueRef ref);
const char* LLDB_SBValue_GetTypeName(SBValueRef ref);
const char* LLDB_SBValue_GetDisplayTypeName(SBValueRef ref);
const char* LLDB_SBValue_GetValue(SBValueRef ref);
bool        LLDB_SBValue_IsInScope(SBValueRef ref);
uint64_t    LLDB_SBValue_GetValueAsUnsigned(SBValueRef ref, uint64_t fail_value);
int64_t     LLDB_SBValue_GetValueAsSigned(SBValueRef ref, int64_t fail_value);
uint64_t    LLDB_SBValue_GetAddress(SBValueRef ref);
uint32_t    LLDB_SBValue_GetNumChildren(SBValueRef ref);
SBValueRef  LLDB_SBValue_GetChildAtIndex(SBValueRef ref, uint32_t idx);
SBValueRef  LLDB_SBValue_GetChildMemberWithName(SBValueRef ref, const char* name);
SBValueRef  LLDB_SBValue_Dereference(SBValueRef ref);
SBErrorRef  LLDB_SBValue_GetError(SBValueRef ref);

/* =========================================================================
 * SBFileSpec
 * ========================================================================= */
SBFileSpecRef LLDB_SBFileSpec_Create(const char* path, bool resolve);
void          LLDB_SBFileSpec_Destroy(SBFileSpecRef ref);
bool          LLDB_SBFileSpec_IsValid(SBFileSpecRef ref);
const char*   LLDB_SBFileSpec_GetFilename(SBFileSpecRef ref);
const char*   LLDB_SBFileSpec_GetDirectory(SBFileSpecRef ref);
bool          LLDB_SBFileSpec_Exists(SBFileSpecRef ref);

/* =========================================================================
 * SBListener
 * ========================================================================= */
SBListenerRef LLDB_SBListener_Create(const char* name);
void          LLDB_SBListener_Destroy(SBListenerRef ref);
bool          LLDB_SBListener_IsValid(SBListenerRef ref);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* LLDB_C_H */
